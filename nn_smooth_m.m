function sY = nn_smooth_m(Y, n)
%UNTITLED Summary of this function goes here
%   Detailed explanation goes here

w = 2 * n + 1;

if w < 1
    error('Smoothing window cannot be less that 1 point')
end

if length(Y) < w
    error('Smoothing window is larger than the available data array');
end

sY = zeros(length(Y) - w + 1, 1);

for i = 1:length(sY)
    sY(i) = mean(Y(i:i+w-1));
end

end

