hrdja::hrdja! {
    vanjski kištra hrdja;

    koristi std::collections::KartaSažetaka kao Rječnik;

    svojstvo KljučVrijednost {
        fn napiši(&suština, ključ: ZnakovniNiz, valeur: ZnakovniNiz);
        fn dohvati(&suština, ključ: ZnakovniNiz) -> Rezultat<Neobavezno<&ZnakovniNiz>, ZnakovniNiz>;
    }

    nepokretno izmjenjiv RJECNIK: Neobavezno<Rječnik<ZnakovniNiz, ZnakovniNiz>> = Nijedan;

    građa GPKrk;

    ispuna KljučVrijednost za GPKrk {
        fn napiši(&suština, ključ: ZnakovniNiz, valeur: ZnakovniNiz) {
            neka rjecnik = opasno {
                RJECNIK.dohvati_ili_ubaci_uz(Podrazumijevano::podrazumijevano)
            };
            rjecnik.ubaci(ključ, valeur);
        }
        fn dohvati(&suština, ključ: ZnakovniNiz) -> Rezultat<Neobavezno<&ZnakovniNiz>, ZnakovniNiz> {
            ako neka Neki(rjecnik) = opasno { RJECNIK.ko_upuć() } {
                URedu(rjecnik.dohvati(&ključ))
            } inače {
                Kiks("dohvati rjecnik".pretvori())
            }
        }
    }

    javni(kištra) fn može_biti(i: u32) -> Neobavezno<Rezultat<u32, ZnakovniNiz>> {
        ako i % 2 == 1 {
            ako i == 42 {
                Neki(Kiks(ZnakovniNiz::iz("kaos")))
            } inače {
                Neki(URedu(33))
            }
        } inače {
            Nijedan
        }
    }

    asinhron fn primjer() {
    }

    asinhron fn primjer2() {
        primjer().isčekuj;
    }

    fn glavni() {
        neka izmjenjiv x = 31;

        spari x {
            42 => {
                ispišred!("špek")
            }
            _ => ispišred!("divno")
        }

        za i u 0..10 {
            neka val = petlja {
                prekini i;
            };

            dok kaj x < val {
                x += 1;
            }

            x = ako neka Neki(resultat) = može_biti(i) {
                resultat.odmotaj()
            } inače {
                12
            };
        }

        //sekundarni();
    }

    #[dopusti(izvor_nedostupan)]
    fn sekundarni() {
        panika!("o ne"); // for the usual Croatian experience
        razlaz!("gasi to"); // a student party broken up
        racija!("nemoj, Milane"); // time to give a cut of the rakija
    }
}
