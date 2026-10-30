# P3 instruction gate

Median of three `instructions:u` samples; 100% counter running. Lower is better.

| Function | / v0.2.15 default real/corpus | Scalar / scalar C++ real/corpus | Default / default C++ real/corpus | Worst / base | Verdict |
|---|---:|---:|---:|---:|---|
| meshlets | 1.0000/1.0000 | 0.9727/1.0196 | 0.9727/1.0195 | 1.0001 | NO-GAIN |
| meshlets-flex | 1.0000/1.0000 | 0.9733/1.0197 | 0.9733/1.0196 | 1.0000 | NO-GAIN |
| cluster-lod | 0.9987/0.9933 | 1.1376/1.1135 | 1.1364/1.1090 | 0.9987 | NO-GAIN |
| cluster-bounds | 0.9316/0.8890 | 1.1322/1.1273 | 1.0749/1.0549 | 0.9323 | SHIPS |
| meshlet-bounds | 0.9327/0.8920 | 1.1437/1.1505 | 1.0868/1.0786 | 0.9333 | SHIPS |

## Every case

| Case | Function | Candidate / base default | Scalar / scalar C++ | Default / default C++ |
|---|---|---:|---:|---:|
| tiny-smooth | meshlets | 1.0001 | 1.2649 | 1.2647 |
| tiny-smooth | meshlets-flex | 0.9999 | 1.2643 | 1.2642 |
| tiny-smooth | cluster-lod | 0.9784 | 1.2346 | 1.2175 |
| tiny-smooth | cluster-bounds | 0.8724 | 1.1940 | 1.0943 |
| tiny-smooth | meshlet-bounds | 0.8746 | 1.2064 | 1.1074 |
| medium-smooth | meshlets | 1.0000 | 0.9628 | 0.9628 |
| medium-smooth | meshlets-flex | 1.0000 | 0.9629 | 0.9629 |
| medium-smooth | cluster-lod | 0.9970 | 1.0764 | 1.0747 |
| medium-smooth | cluster-bounds | 0.8918 | 1.1101 | 1.0433 |
| medium-smooth | meshlet-bounds | 0.8950 | 1.1361 | 1.0698 |
| medium-seams | meshlets | 1.0000 | 0.9577 | 0.9577 |
| medium-seams | meshlets-flex | 1.0000 | 0.9577 | 0.9577 |
| medium-seams | cluster-lod | 0.9969 | 1.0966 | 1.0948 |
| medium-seams | cluster-bounds | 0.8969 | 1.1198 | 1.0552 |
| medium-seams | meshlet-bounds | 0.8998 | 1.1444 | 1.0804 |
| medium-sparse | meshlets | 1.0000 | 0.9644 | 0.9644 |
| medium-sparse | meshlets-flex | 1.0000 | 0.9644 | 0.9644 |
| medium-sparse | cluster-lod | 0.9971 | 1.0825 | 1.0808 |
| medium-sparse | cluster-bounds | 0.8918 | 1.1101 | 1.0433 |
| medium-sparse | meshlet-bounds | 0.8950 | 1.1361 | 1.0698 |
| million-smooth | meshlets | 1.0000 | 0.9795 | 0.9795 |
| million-smooth | meshlets-flex | 1.0000 | 0.9803 | 0.9803 |
| million-smooth | cluster-lod | 0.9973 | 1.0850 | 1.0835 |
| million-smooth | cluster-bounds | 0.8923 | 1.1050 | 1.0394 |
| million-smooth | meshlet-bounds | 0.8955 | 1.1311 | 1.0661 |
| real-214e | meshlets | 1.0000 | 0.9714 | 0.9714 |
| real-214e | meshlets-flex | 1.0000 | 0.9720 | 0.9720 |
| real-214e | cluster-lod | 0.9987 | 1.1384 | 1.1372 |
| real-214e | cluster-bounds | 0.9323 | 1.1333 | 1.0755 |
| real-214e | meshlet-bounds | 0.9333 | 1.1441 | 1.0865 |
| real-a082 | meshlets | 1.0000 | 0.9713 | 0.9713 |
| real-a082 | meshlets-flex | 1.0000 | 0.9718 | 0.9718 |
| real-a082 | cluster-lod | 0.9986 | 1.1376 | 1.1364 |
| real-a082 | cluster-bounds | 0.9313 | 1.1315 | 1.0745 |
| real-a082 | meshlet-bounds | 0.9324 | 1.1434 | 1.0867 |
| real-b6fd | meshlets | 1.0000 | 0.9756 | 0.9756 |
| real-b6fd | meshlets-flex | 1.0000 | 0.9762 | 0.9762 |
| real-b6fd | cluster-lod | 0.9986 | 1.1369 | 1.1357 |
| real-b6fd | cluster-bounds | 0.9313 | 1.1317 | 1.0748 |
| real-b6fd | meshlet-bounds | 0.9324 | 1.1437 | 1.0870 |

## Wall time disclosure

Resident-call seconds, median of three. Shared-machine observations, not the gate. Bound queries use frozen baseline meshlets (64 vertices, 124 triangles maximum), localized outside counts. Vertex/index inputs are shared across arms; output collection is included for every arm.

| Case | Function | Base default s | Candidate default s | Candidate scalar s | C++ scalar s | C++ default s |
|---|---|---:|---:|---:|---:|---:|
| tiny-smooth | meshlets | 0.00000518 | 0.00000518 | 0.00000520 | 0.00000458 | 0.00000481 |
| tiny-smooth | meshlets-flex | 0.00000527 | 0.00000520 | 0.00000521 | 0.00000454 | 0.00000479 |
| tiny-smooth | cluster-lod | 0.00000990 | 0.00000951 | 0.00000971 | 0.00000977 | 0.00000941 |
| tiny-smooth | cluster-bounds | 0.00000158 | 0.00000134 | 0.00000154 | 0.00000143 | 0.00000136 |
| tiny-smooth | meshlet-bounds | 0.00000160 | 0.00000136 | 0.00000157 | 0.00000132 | 0.00000131 |
| medium-smooth | meshlets | 0.00354192 | 0.00324536 | 0.00330878 | 0.00389947 | 0.00411443 |
| medium-smooth | meshlets-flex | 0.00342144 | 0.00328647 | 0.00327830 | 0.00384278 | 0.00391164 |
| medium-smooth | cluster-lod | 0.01361416 | 0.01388957 | 0.01371858 | 0.01418556 | 0.01425800 |
| medium-smooth | cluster-bounds | 0.00034073 | 0.00029029 | 0.00030571 | 0.00033813 | 0.00033947 |
| medium-smooth | meshlet-bounds | 0.00031840 | 0.00027921 | 0.00030560 | 0.00035222 | 0.00033552 |
| medium-seams | meshlets | 0.00232089 | 0.00234654 | 0.00236836 | 0.00273294 | 0.00291727 |
| medium-seams | meshlets-flex | 0.00247858 | 0.00234699 | 0.00234276 | 0.00262395 | 0.00283261 |
| medium-seams | cluster-lod | 0.01330221 | 0.01285939 | 0.01352639 | 0.01272798 | 0.01340695 |
| medium-seams | cluster-bounds | 0.00031883 | 0.00027707 | 0.00030550 | 0.00030709 | 0.00030887 |
| medium-seams | meshlet-bounds | 0.00032839 | 0.00029673 | 0.00034239 | 0.00031250 | 0.00031187 |
| medium-sparse | meshlets | 0.00351483 | 0.00347039 | 0.00347999 | 0.00397971 | 0.00409909 |
| medium-sparse | meshlets-flex | 0.00354423 | 0.00344070 | 0.00350403 | 0.00398783 | 0.00406438 |
| medium-sparse | cluster-lod | 0.01535215 | 0.01476366 | 0.01408494 | 0.01419253 | 0.01452402 |
| medium-sparse | cluster-bounds | 0.00038501 | 0.00031814 | 0.00037757 | 0.00035863 | 0.00038065 |
| medium-sparse | meshlet-bounds | 0.00045522 | 0.00030242 | 0.00032450 | 0.00035953 | 0.00035272 |
| million-smooth | meshlets | 0.53238843 | 0.52350204 | 0.52163229 | 0.65920544 | 0.64369094 |
| million-smooth | meshlets-flex | 0.51980257 | 0.51777161 | 0.51605600 | 0.59462733 | 0.60256202 |
| million-smooth | cluster-lod | 2.09060075 | 2.04562287 | 2.02035270 | 1.96882523 | 2.03946699 |
| million-smooth | cluster-bounds | 0.05986764 | 0.04134284 | 0.04451134 | 0.04933643 | 0.04884707 |
| million-smooth | meshlet-bounds | 0.04904013 | 0.04071638 | 0.04610426 | 0.05219514 | 0.05172719 |
| real-214e | meshlets | 3.06557866 | 3.10932589 | 3.16741890 | 3.21692846 | 3.43537791 |
| real-214e | meshlets-flex | 3.30652729 | 3.26853782 | 3.49026552 | 3.51892663 | 3.26463242 |
| real-214e | cluster-lod | 29.19161884 | 27.86456415 | 27.41629252 | 72.96661296 | 79.96636895 |
| real-214e | cluster-bounds | 0.51001218 | 0.45217916 | 0.51520573 | 0.67765747 | 0.67226511 |
| real-214e | meshlet-bounds | 0.48437961 | 0.43226923 | 0.46555138 | 0.63131816 | 0.64996879 |
| real-a082 | meshlets | 6.74798010 | 6.50498765 | 6.94397477 | 6.69814604 | 6.97769268 |
| real-a082 | meshlets-flex | 6.78698167 | 6.83497875 | 6.74497042 | 6.55318853 | 6.94283449 |
| real-a082 | cluster-lod | 19.83388082 | 20.27723274 | 20.71219794 | 20.71959485 | 20.77505093 |
| real-a082 | cluster-bounds | 0.44371388 | 0.37064324 | 0.36254533 | 0.60336286 | 0.59337251 |
| real-a082 | meshlet-bounds | 0.35898809 | 0.31231711 | 0.34934484 | 0.59001393 | 0.55377356 |
| real-b6fd | meshlets | 1.30956225 | 1.20389274 | 1.20904651 | 1.38328989 | 1.30402096 |
| real-b6fd | meshlets-flex | 1.42705357 | 1.26783100 | 1.18700730 | 1.28700527 | 1.31986584 |
| real-b6fd | cluster-lod | 4.91299067 | 4.79901757 | 4.67480105 | 4.53397941 | 4.59898510 |
| real-b6fd | cluster-bounds | 0.08620470 | 0.07001737 | 0.07919323 | 0.12901543 | 0.13037762 |
| real-b6fd | meshlet-bounds | 0.07875602 | 0.07250705 | 0.07659579 | 0.12930769 | 0.14016572 |
