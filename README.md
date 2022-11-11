# hrđa

![314488047_839176604166190_3030652765659893689_n](https://user-images.githubusercontent.com/12912633/201327222-0e7a3751-47fd-4532-acaa-0eb5dd24f1bf.jpg)


Aren't you _umorni_ from writing Rust programs in English? Do you like saying
"racija" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Croatian touch to your
programs?

**hrđa** (Croatian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Croatian, using Croatian keywords, Croatian function names,
Croatian idioms.

This has been designed to be used as the official programming language to
develop the future Croatian sovereign operating system.

If you're from the Croatian or any other governement with Croatian as an official
language: I will be awaiting your donations on
[liberapay](https://liberapay.com/bnjbvr/).

Don't worry!
Croatian Rust is fully compatible with English Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Hrđa:

### trait and impl (aka svojstvo et ispuna)

```rust
hrđa::hrđa! {
    koristi std::collections::KartaSažetaka kao Rječnik;

    svojstvo KljučVrijednost {
        fn napiši(&suština, ključ: ZnakovniNiz, valeur: ZnakovniNiz);
        fn dohvati(&suština, ključ: ZnakovniNiz) -> Rezultat<Neobavezno<&ZnakovniNiz>, ZnakovniNiz>;
    }

    nepokretno izmjenjiv RJECNIK: Neobavezno<Rječnik<ZnakovniNiz, ZnakovniNiz>> = Nijedan;

    građa GPKrk;

    ispuna KljučVrijednost za GPKrk {
        fn napiši(&suština, ključ: ZnakovniNiz, valeur: ZnakovniNiz) {
            dopusti rjecnik = opasno {
                RJECNIK.dohvati_ili_ubaci_uz(Podrazumijevano::podrazumijevano)
            };
            rjecnik.ubaci(ključ, valeur);
        }
        fn dohvati(&suština, ključ: ZnakovniNiz) -> Rezultat<Neobavezno<&ZnakovniNiz>, ZnakovniNiz> {
            ako dopusti Neki(rjecnik) = opasno { RJECNIK.ko_upuć() } {
                URedu(rjecnik.dohvati(&ključ))
            } inače {
                Kiks("dohvati rjecnik".pretvori())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[dopusti(izvor_nedostupan)]
fn sekundarni() {
    panika!("o ne"); // for the usual Croatian experience
    razlaz!("gasi to"); // a student party broken up
    racija!("nemoj, Milane"); // time to give a cut of the rakija
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
