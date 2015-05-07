clear;
clf;

M = csvread('/home/ed/3rd-year-project/final-report/results/100-200 list/output2991.csv');

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
clearvars M;
n=0.1;
for i = 1:size(X,3)
    n=4294967295.00000;
    A = X(X(:,1,i) ~= n,1,i);
    
    B=A(A<mean(A)+n*std(A));
    C=B(B>mean(A)-n*std(A));

    
   meanlist(i) = mean(C);
    
end

plot(meanlist);

print -dpng fitness.png
