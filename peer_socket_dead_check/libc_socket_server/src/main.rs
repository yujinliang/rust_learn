use libc;
use std::io::{Error, ErrorKind};
use std::mem;
use std::ffi::c_void;

fn main()  {

    unsafe {

        let socket = libc::socket(libc::AF_INET, libc::SOCK_STREAM, libc::IPPROTO_TCP);
        if socket < 0 {
            panic!("last OS error: {:?}", Error::last_os_error());
        }

        let servaddr = libc::sockaddr_in {
            sin_family: libc::AF_INET as u16,
            sin_port: 8080u16.to_be(),
            sin_addr: libc::in_addr {
                s_addr: u32::from_be_bytes([127, 0, 0, 1]).to_be()
            },
            sin_zero: mem::zeroed()
        };

        let result = libc::bind(socket, &servaddr as *const libc::sockaddr_in as *const libc::sockaddr, mem::size_of_val(&servaddr) as u32);
        if result < 0 {
            println!("last OS error: {:?}", Error::last_os_error());
            libc::close(socket);
        }

        libc::listen(socket, 128);

        loop {

            let mut peeraddr: libc::sockaddr_storage = mem::zeroed();
            let mut len = mem::size_of_val(&peeraddr) as u32;

            let peer_socket = libc::accept(socket, &mut peeraddr as *mut libc::sockaddr_storage as *mut libc::sockaddr, &mut len);
            if peer_socket < 0 {
                println!("last OS error: {:?}", Error::last_os_error());
                break;
            }

            //handle peer socket.
            rayon::spawn(move || {

                    let mut buf = [1u8; 20];
                    loop {

                        let n = libc::read(peer_socket, &mut buf as *mut _ as *mut c_void, buf.len());

                       /* if  n <= 0  {
                            println!("read , n: {} , last OS error: {:?}", n, Error::last_os_error());
                            libc::close(peer_socket);
                            break;
                        }*/

                        //mean peer socket is to be closed.
                        if n == 0 {
                            //remove /close the peer socket
                            println!("read , n=0: {} , last OS error: {:?}", n, Error::last_os_error());
                             libc::close(peer_socket);
                            break;
                        } else if n < 0 {
                            println!("read , n<0: {} , last OS error: {:?}", n, Error::last_os_error());
                            //error occurred, peer socket is dead?
                            match Error::last_os_error().kind() {
                                ErrorKind::ConnectionRefused | 
                                ErrorKind::ConnectionReset | 
                                ErrorKind::ConnectionAborted |
                                 ErrorKind::NotConnected  => {
                                    libc::close(peer_socket);
                                    break;
                                 },
                                 //ErrorKind::Interrupted  => continue,  //ignore signal interrupted.
                                _ => continue,
                            }
                        }
                      
                        println!("{:?}", String::from_utf8_lossy(&buf)); 

                        //send back to peer
                        let nleft = buf.len();
                        let bufp = &buf as *const _ as *const u8;
                        let nwritten = rio_writen(peer_socket, bufp, nleft);
                        println!("write , n: {} , last OS error: {:?}", nwritten, Error::last_os_error());
                        if nwritten < 0 {
                            //what error is to closed socket.
                            match Error::last_os_error().kind() {
                                ErrorKind::BrokenPipe => {
                                    //means the peer socket is dead!
                                    libc::close(peer_socket);
                                    break;
                                }
                                //ErrorKind::Interrupted  => continue,  //ignore signal interrupted.
                                _ => continue,
                            }
                        }

                    }
            });

        } //end accept loop.
    }
}

fn rio_writen(fd:i32,  userbuf:*const u8, n:usize) -> isize {

    unsafe {
        let mut nleft = n;
        let mut  bufp = userbuf;
        let mut nwritten = 0isize;       
        while nleft > 0 {
                        nwritten = libc::write(fd, bufp as *const c_void, nleft);
                        //attention: nwritten hit 0, that means maybe some wrong or nothing, but  always retry to write again and agian , until errors occureed(-1) or all Ok.
                        if  nwritten < 0 {
                                if Error::last_os_error().kind() == ErrorKind::Interrupted {
                                        nwritten = 0;
                                } else {
                                        return -1;
                               }
                         }
                        nleft -= nwritten as usize;
                        bufp = bufp.offset(nwritten);
             }
        }
            return n as isize;
}