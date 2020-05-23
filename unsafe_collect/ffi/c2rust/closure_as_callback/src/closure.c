typedef void (*AddCallback)(int result, void *user_data);

void better_add_two_numbers(int a, int b, AddCallback cb, void *user_data)
{
    int result = a + b;
    cb(result, user_data);
}