addpath('example_rustlib/target/release');
[notfound, warnings] = loadlibrary('example_rustlib', 'example_rustlib.h', 'alias', 'rustlib');

if libisloaded('rustlib')
    calllib('rustlib', 'hello', 'from Rust')
    calllib('rustlib', 'add', 2.0, 3.0)
    p = calllib('rustlib', 'create_vec3');
    calllib('rustlib', 'free_vec3', p);
    clear p;
    unloadlibrary('rustlib');
else
    fprintf('**Library load failed**');
end
