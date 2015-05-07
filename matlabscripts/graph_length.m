clear;
clf;

M = csvread('output46.csv');

length = size(M);
columns=4;

popcount=length(2)/columns;

X = zeros(length(2)/columns,columns,length(1));

i=2;

for i = 1:length(1)
    for j = 1:columns:length(2)
        X(((j-1)/columns)+1,:,i)= M(i,j:j+columns-1)';
        
    end
    
end

for i = 1:size(X,3)


   average_length(i) = mean(X(:,2,i));
    
end

plot(average_length);

print -dpng length.png
