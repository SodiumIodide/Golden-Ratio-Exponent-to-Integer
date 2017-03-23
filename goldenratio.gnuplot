set termoption dashed
set termoption enhanced
set tics front
reset
set size 7.500000000000e-1,1.000000000000e0
set title "Golden Ratio to an Integer Power, Distance From the Nearest Integer"
set key at  graph 1.0000000000000000e0, graph 5.0000000000000000e-1 left center vertical noreverse noinvert Right
set border 3 front  lc rgb "black" lw 2.000000000000e0 lt 1
set xlabel "n"
set ylabel "phi^n - nearest integer"
unset logscale x
set mxtics 2

set xrange [*:*]
set xtics  1.000000000000e1 nomirror scale 5.000000000000e-1,5.000000000000e-1
unset logscale y
set mytics 2

set yrange [*:*]
set ytics  1.000000000000e-1 nomirror scale 5.000000000000e-1,5.000000000000e-1
unset logscale cb
unset mcbtics

set cbrange [*:*]
plot "-" binary endian=little record=60 format="%float64" using 1:2 with lines lw 1.500000000000e0 lt 1 lc rgb "blue" t "Distance"
      �?`-��!rؿ       @`-��!rؿ      @�J�y7�?      @@�iɬ¿      @�t|�`�?      @ Xhd���      @ ���\��?       @ �;"̕�      "@  ?U�?      $@  8-ɦ��      &@ ���t?      (@  ��pi�      *@  �6ur_?      ,@  ��toS�      .@  �� H?      0@   <ұ=�      1@  �/Z2?      2@   PF�&�      3@   �/
?      4@   �\T�      5@   `�k?      6@    )z��      7@    "]�>      8@    :�      9@    h �>      :@    `�ξ      ;@    ��>      <@     ���      =@     .�>      >@     
��      ?@     H�>      @@     ���     �@@      �>      A@     �u�     �A@      j>      B@      b�     �B@      P>      C@      P�     �C@              D@      `�     �D@      p�      E@      ��     �E@              F@      ��     �F@      ��      G@             �G@      ��      H@      ��     �H@      о      I@      о     �I@              J@      �     �J@       �      K@       �     �K@      �      L@      �     �L@       �      M@      0�     �M@      0�      N@      @�