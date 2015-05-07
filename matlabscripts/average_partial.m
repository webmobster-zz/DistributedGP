clf;
clearvars -except X;
Z=X(:,:,[1:115 117:end]);

z=0.5;
q=1;
p=size(Z,3)-1000;
o=1;

for i = q:p-1

   %n=4294967295.00000;
   %A = X(X(:,1,i) ~= n,1,i);
   %A=log(A);

   n=4194967295.00000;
   A = Z(:,1,i);
   B = Z(:,2,i);
   %remove length
   
   for k= 1:size(A,1)
         if B(k) > 500
            C(k)= A(k) - 10000*B(k);
         else
            C(k)= A(k) - B(k);
         end

         
   end
   
   

  % Z = B(B>500)
   C = C(C(:) < n);


   D=C(C<mean(A)+z*std(C));
   E=D(D>mean(C)-z*std(C));
   %E=C;
   %E=E*1*10^-4;
   %b = mod(i,5);
   movingaverage(mod(i-1,5)+1)=median(E)/10000;
   movingaveragetwo(mod(i-1,5)+1)=mean(E)/10000;

   meanlist(o) = mean(movingaverage);
   meanlisttwo(o) = mean(B);
   o=o+1;
    
end
hold on;
x=linspace(q,p,p-q);

%plot(meanlisttwo);
%plot(meanlisttwo,'-.');

[hAx,hLine1,hLine2]=plotyy(x,meanlist,x,meanlisttwo);
legend('Moving Average of Median Fitness','Mean Size');
hLine1.LineStyle = '-';
hLine2.LineStyle = '-.';
%plot(meanlisttwo);
ylabel(hAx(1),'Median Fitness') % left y-axis
ylabel(hAx(2),'Mean Size') % right y-axis
xlabel('Generation');
%ylabel('Fitness as a Percentage of Remaining Inversions');



print -dpng size-hundred.png
