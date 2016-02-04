% Speed test between nn_smooth implemented in rust vs. matlab

n = 1e5;
w = randn(n, 1);
n_neighbors = 100;

n_runs = 1000;
tic;
for i = 1:n_runs
    sm_rust_w = nn_smooth(w, n_neighbors);
end
dt = toc;
fprintf('---------------------------------------\n');
fprintf('Execution time for nn_smooth (in rust): %.3f ms\n', dt);

tic;
sm_matlab_w = nn_smooth_m(w, n_neighbors);
dt = toc;
fprintf('---------------------------------------\n');
fprintf('Execution time for nn_smooth_m (pure matlab): %.3f ms\n', dt * 1000);

hold on;
plot(1:n, sm_rust_w);
plot(n_neighbors + (1:length(sm_matlab_w)), sm_matlab_w);
hold off;
