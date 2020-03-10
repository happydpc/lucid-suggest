const compile = require('../build/index')


describe('Suggest', () => {
    const records = [
        {id: 10, text: 'Hello world!'},
        {id: 20, text: 'Foo bar'},
        {id: 30, text: '-BAZZZ-'},
    ]

    let suggest = null

    beforeAll(() => {
        return compile.then(LucidSuggest => {
            suggest = new LucidSuggest()
            suggest.addRecords(records)
        })
    })

    test('Empty input', () => {
        const hits = suggest.search('')
        expect(hits).toMatchSnapshot()
    })

    test('Equality', () => {
        const hits = suggest.search('foo bar')
        expect(hits).toMatchSnapshot()
    })

    test('Prefix', () => {
        const hits = suggest.search('ba')
        expect(hits).toMatchSnapshot()
    })

    test('Prio', () => {
        return compile.then(LucidSuggest => {
            suggest = new LucidSuggest()
            suggest.addRecords(records.map((r, i) => ({...r, prio: i})))
            const hits = suggest.search('')
            expect(hits).toMatchSnapshot()
        })
    })
})