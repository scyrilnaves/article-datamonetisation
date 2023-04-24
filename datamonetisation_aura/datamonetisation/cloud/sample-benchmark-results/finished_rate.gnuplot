# set terminal pngcairo size 600,400
set output "finished_and_invalid_rate.eps"

set terminal postscript eps enhanced color font 'Times-Roman,18' size 9,10

set multiplot layout 4,1 

######################################

set title "Finished rate for 5 to 25 nodes"

set grid ytics lc rgb "black" lw 1.5 lt 0.1
set grid xtics lc rgb "black" lw 1.5 lt 0.1


set xlabel "Input TPS"
set ylabel "Finished TPS"

# set yrange [0 to 2500]
set xtics ("200" 200, "400" 400, "600" 600, "800" 800, "1000" 1000, "1200" 1200, "1400" 1400, "1600" 1600, "2000" 2000, "2500" 2500)

set key at graph 0.15, 0.95

#csv settings:
set datafile separator ","


$data << EOD
200,129.9,125,121.6,120.9,121.8,120.7,122.1,132.5
400,320.2,318.5,321.2,321.3,289.4,321.2,325.3,330.7
600,387.6,388.7,394,395.8,428.2,394.4,406.9,530.2
800,442.3,443.6,463.8,476,485.8,504.7,634.7,318.2
1000,563.3,675.5,753,714.3,801.5,899.6,264,1042.7
1200,777.4,834.6,855.4,902.4,962.5,984.5,1030,
1400,774,881.4,861.6,817,872.2,375.8,689.4,1100
1600,682,776.2,777.9,939,916.2,812.6,1103.3,1007.8
2000,630.8,767.1,811.3,728.1,793.8,988.5,1114.1,808.9
2500,544.6,626.8,763.7,736,1018.8,1099.8,987.9,

EOD

plot "$data" using 1:2 with linespoints lw 2 title "5 nodes", \
    "$data" using 1:3 with linespoints lw 2 title "7 nodes", \
    "$data" using 1:4 with linespoints lw 2 title "9 nodes", \
    "$data" using 1:5 with linespoints lw 2 title "10 nodes", \
    "$data" using 1:6 with linespoints lw 2 title "12 nodes", \
    "$data" using 1:7 with linespoints lw 2 title "15 nodes", \
    "$data" using 1:8 with linespoints lw 2 title "20 nodes", \
    "$data" using 1:9 with linespoints lw 2 title "25 nodes"


######################################

set title "Invalid rate for 5 to 25 nodes"

set grid ytics lc rgb "black" lw 1.5 lt 0.1
set grid xtics lc rgb "black" lw 1.5 lt 0.1


set xlabel "Input TPS"
set ylabel "Invalid TPS"

# set yrange [0 to 2500]
set xtics ("200" 200, "400" 400, "600" 600, "800" 800, "1000" 1000, "1200" 1200, "1400" 1400, "1600" 1600, "2000" 2000, "2500" 2500)

set key at graph 0.15, 0.95

#csv settings:
set datafile separator ","

$data << EOD
200,0,0,0,0,0,0,0,0
400,0,0,0,0,0.1,0,0,0
600,0,0,0,0,0.1,0,0,1.4
800,0,0,0,0.3,0,0,1.2,2.5
1000,0,1.1,2.8,2.1,4.9,8.2,2.5,13.7
1200,3.7,7,7.4,9.3,10.9,10.6,13.6,
1400,5.8,7.2,9,6.9,9.5,4,8.8,16
1600,5,6.6,8,12.1,9.5,9.8,17.1,14.1
2000,1.3,7.9,8.8,6.6,7.5,10.8,16.5,11.8
2500,0.8,4.6,6.4,5.8,10.9,13.1,11.3,

EOD

plot "$data" using 1:2 with linespoints lw 2 title "5 nodes", \
    "$data" using 1:3 with linespoints lw 2 title "7 nodes", \
    "$data" using 1:4 with linespoints lw 2 title "9 nodes", \
    "$data" using 1:5 with linespoints lw 2 title "10 nodes", \
    "$data" using 1:6 with linespoints lw 2 title "12 nodes", \
    "$data" using 1:7 with linespoints lw 2 title "15 nodes", \
    "$data" using 1:8 with linespoints lw 2 title "20 nodes", \
    "$data" using 1:9 with linespoints lw 2 title "25 nodes"


######################################

set title "Finished rate variance for 5 to 25 nodes"

set grid ytics lc rgb "black" lw 1.5 lt 0.1
set grid xtics lc rgb "black" lw 1.5 lt 0.1


set xlabel "Input TPS"
set ylabel "Finished Rate Variance"

# set yrange [0 to 2500]
set xtics ("200" 200, "400" 400, "600" 600, "800" 800, "1000" 1000, "1200" 1200, "1400" 1400, "1600" 1600, "2000" 2000, "2500" 2500)

set key at graph 0.15, 0.95

#csv settings:
set datafile separator ","

$data << EOD
200,13625.1,12685.7,12146.5,12023.2,12230.5,12125.6,12652.5,14683
400,160784.2,162361,163510.1,163522.3,153322.2,165919.9,166890,171939.9
600,336682.5,340711.4,346612.7,346336.7,400399.8,348996.6,369215.9,438044.9
800,529533.1,541444.4,569816,639358.6,618750.3,651782.7,835653.8,476756.4
1000,882013.6,1136822.7,1277317.5,1165240.2,1272557.9,1446721.4,525133.5,1204641.7
1200,1454155.4,1527706.2,1451325.2,1503156.4,1654469.6,1617781.5,1490710.1,
1400,1418642.5,1595332.3,1510776.9,1320889.7,1414667.3,809049.3,1156799.5,1284690.7
1600,1259843.6,1347674,1242400,1690867.6,1589070.2,1609329.4,1574076.9,1192342.1
2000,1086383.1,1327598.4,1477678.8,1216336.5,1255773.4,1628149.5,1400009.2,1121221.9
2500,871805.8,1011058.4,1273422.8,1174852.8,1744909.9,1844644.1,1279208.4,

EOD

plot "$data" using 1:2 with linespoints lw 2 title "5 nodes", \
    "$data" using 1:3 with linespoints lw 2 title "7 nodes", \
    "$data" using 1:4 with linespoints lw 2 title "9 nodes", \
    "$data" using 1:5 with linespoints lw 2 title "10 nodes", \
    "$data" using 1:6 with linespoints lw 2 title "12 nodes", \
    "$data" using 1:7 with linespoints lw 2 title "15 nodes", \
    "$data" using 1:8 with linespoints lw 2 title "20 nodes", \
    "$data" using 1:9 with linespoints lw 2 title "25 nodes"



######################################

set title "Max finished rate for 5 to 25 nodes"

set grid ytics lc rgb "black" lw 1.5 lt 0.1
set grid xtics lc rgb "black" lw 1.5 lt 0.1


set xlabel "Input TPS"
set ylabel "Max Finished Rate"

# set yrange [0 to 2500]
set xtics ("200" 200, "400" 400, "600" 600, "800" 800, "1000" 1000, "1200" 1200, "1400" 1400, "1600" 1600, "2000" 2000, "2500" 2500)

set key at graph 0.15, 0.95

#csv settings:
set datafile separator ","

$data << EOD
200,262.8,247.3,245.7,241.2,242.2,238.5,257.8,291.2
400,934.4,940.9,951,956.4,958.5,971.6,964.5,1015.7
600,1500.2,1545.1,1565.6,1556.6,1754.6,1617,1632.2,1825.6
800,2138,2153.4,2199.2,2438.4,2281.7,2349.8,2526.9,2628.3
1000,2946.3,3085.2,3363.2,3225.3,3392.3,3643.8,3241.4,3040.1
1200,3441.4,3684.2,3519.6,3674.7,3841.8,3644.9,3521.2,
1400,3480.6,3796.5,3725.3,3400.8,3536.1,3538,3421,3294.7
1600,3375.9,3402.8,3206.9,3818.7,3797.3,3944.4,3597.3,3164
2000,3092.5,3409.8,3609.6,3449.7,3323.7,3603.8,3371.5,3058.6
2500,2926.5,3013.1,3234.7,3168.2,3608.4,3747.4,3147.2,

EOD

plot "$data" using 1:2 with linespoints lw 2 title "5 nodes", \
    "$data" using 1:3 with linespoints lw 2 title "7 nodes", \
    "$data" using 1:4 with linespoints lw 2 title "9 nodes", \
    "$data" using 1:5 with linespoints lw 2 title "10 nodes", \
    "$data" using 1:6 with linespoints lw 2 title "12 nodes", \
    "$data" using 1:7 with linespoints lw 2 title "15 nodes", \
    "$data" using 1:8 with linespoints lw 2 title "20 nodes", \
    "$data" using 1:9 with linespoints lw 2 title "25 nodes"



######################################


unset multiplot 
