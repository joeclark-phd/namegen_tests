#namegen_tests

This repo contains a sample Rust program to run some procedural generation of random text using my `markov_namegen` crate.  It also includes (in the `resources` directory) several files containing names and other data that you may use to train your own models.

## Training data

Training data files are stored in the `/resources` directory.

A few of these (e.g. `romans.txt`, `vikings_male.txt`, `vikings_female.txt`, `uk_surnames.txt`) I compiled from public sources. Then I discovered and copied over 118 files of sample data from a treasure trove in this [Haxe-based name generator project](https://github.com/Tw1ddle/MarkovNameGenerator) by [Tw1ddle](https://github.com/Tw1ddle), which is under a CC BY-NC 4.0 license.  Lately I discovered a 15th-century list of names for dogs (!) from [this article](https://doi.org/10.1484/J.VIATOR.1.103488) and added that as another resource.

If you'd like to add some data sets of your own, contributions of data are welcome! (As long as you're not violating someone's intellectual property.)  Just fork and open a pull request.

## Example output from CharacterChainGenerator

    Here are some tests of the markov_namegen crate!
    
    10 names based on romans.txt with default settings:
    1360 sequences successfully trained; 0 errors
    rex
    commidus
    arinus
    geneuso
    marcelsrailus
    rufinius
    mellius
    antor
    colactavianus
    arvbillianus
    
    10 more with a pattern that keeps the length from 4-10 characters
    1360 sequences successfully trained; 0 errors
    soterula
    titus
    agatorius
    patidius
    augenna
    druscus
    glycianus
    reginuchus
    janus
    micolumnus
    
    10 more with a pattern that requires they end with -ia
    1360 sequences successfully trained; 0 errors
    liberia
    pria
    frumia
    lurcia
    maximia
    maecia
    romitia
    hybria
    publicia
    ramia
    
    10 names based on uk_surnames.txt
    3706 sequences successfully trained; 0 errors
    yorke
    gouldingston
    white
    wilmont
    toole
    mcclellett
    milley
    mccullor
    mclemague
    mccalf
    
    10 names based on periodic_elements.txt
    109 sequences successfully trained; 0 errors
    sulfugen
    tanium
    norine
    astondelenium
    thanum
    bohrium
    cobalt
    gallium
    yttstenberkum
    protassium

## Example output from ClusterChainGenerator

    Here are some tests of the ClusterChainGenerator!
    
    10 names based on romans.txt with default settings:
    1360 sequences successfully trained; 0 errors
    ligulus
    mercatius
    commius
    sanctus
    favolenus
    roman
    crtricius
    perperna
    scicceianus
    auxientius
    
    10 more with a pattern that keeps the length from 4-10 characters:
    1360 sequences successfully trained; 0 errors
    tibullius
    cotentrva
    tiberius
    publianus
    memor
    vetullio
    carasicus
    novelius
    amantius
    vibennis
    
    10 more with a pattern that requires they end with -ia:
    1360 sequences successfully trained; 0 errors
    glaglycia
    bestia
    damia
    proxsmbria
    fimbria
    porcia
    octaglycia
    bestia
    porcia
    porcia
    
    10 names based on uk_surnames.txt, without priors:
    3706 sequences successfully trained; 0 errors
    vin
    ovegrove
    robins
    jordon
    dunkley
    tomas
    o callam
    paller
    devann
    cas
    
    10 names based on periodic_elements.txt:
    109 sequences successfully trained; 0 errors
    hafniumytterbium
    yttrium
    radolinium
    yttrium
    chrondium
    meitnerium
    lanthandmium
    lithium
    copper
    holmium
    
    10 names based on pokemon_modern.txt (length 6-12):
    742 sequences successfully trained; 0 errors
    claunchlyph
    cottonrkrai
    morkoal
    klefki
    mespriou
    glalie
    rattaqwfblim
    persphy
    behhappiny
    pangorok
