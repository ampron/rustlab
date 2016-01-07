addpath('example_rustlib/target/release');
[notfound, warnings] = loadlibrary('example_rustlib', 'example_rustlib.h', 'alias', 'rustlib');
libfunctions rustlib -full

if libisloaded('rustlib')
    % pass a string
    calllib('rustlib', 'hello', 'from Rust')
    % return a double
    y = calllib('rustlib', 'add', 2.0, 3.0);
    fprintf('y = 2.0 + 3.0 = %f', y);
    % create/destroy a remote struct
    p = calllib('rustlib', 'create_vec3');
    calllib('rustlib', 'free_vec3', p);
    clear p;
    % Pass and manipulate an array
    m = reshape(1:12, 4, 3)
    m_shape = size(m);
    % this function will multiply each element by 3
    m_out = calllib('rustlib', 'mult_array', m, numel(m));
    m_out = reshape(m_out, m_shape)
    % apply nearest-neighbor smoothing to an array
    X = randn(10000, 1);
    smX = calllib('rustlib', 'nn_smooth', X, numel(X), 100);
    figure;
    hold on;
    plot(X, 'o');
    plot((1:10000-200) + 100, smX(101:end-100));
    hold off;
    % Done
    unloadlibrary('rustlib');
else
    fprintf('**Library load failed**');
end
