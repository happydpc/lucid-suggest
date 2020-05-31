const LucidSuggest = require('../build/index')


describe('Spanish language', () => {
    const records = [
        {id: 10, title: 'Pack de 24 pilas alcalinas AA'},
        {id: 20, title: 'Cable de USB A a Lightning'},
        {id: 30, title: 'Cepillo de dientes eléctrico'},
        {id: 40, title: 'Deshumidificador Eléctrico portátil'},
    ]

    const suggest = new LucidSuggest()
    suggest.setLanguage('es')
    suggest.setRecords(records)

    test('Empty input', async () => {
        const hits = await suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', async () => {
        const hits = await suggest.search('cepillo de dientes electrico')
        expect(hits).toMatchSnapshot()
    })

    test('Stemming', async () => {
        const hits = await suggest.search('alcalino ')
        expect(hits).toMatchSnapshot()
    })

    test('Partiles', async () => {
        const hits = await suggest.search('de')
        expect(hits).toMatchSnapshot()
    })
})