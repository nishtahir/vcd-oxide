module Counter;

logic [3:0] count;

initial begin
    $dumpfile("Counter.vcd");
    $dumpvars(0, Counter);
    // count to 5
    #1 count = 0;
    #1 count = 1;
    #1 count = 2;
    #1 count = 3;
    #1 count = 4;
    #1 count = 5;

end

endmodule