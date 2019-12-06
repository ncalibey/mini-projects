<template>
    <div class="container">
        <app-header :quoteCount="quotes.length" :maxQuotes="maxQuotes"></app-header>
        <app-new-quote @quoteAdded="addQuote($event)"></app-new-quote>
        <app-quote-grid :quotes="quotes" @quoteDeleted="deleteQuote($event)"></app-quote-grid>
        <div class="row">
            <div class="col-sm-12 text-center">
                <div class="alert alert-info">Info: Click on a Quote to Delete it!</div>
            </div>
        </div>
    </div>
</template>

<script>
    import QuoteGrid from './components/QuoteGrid.vue';
    import NewQuote from './components/NewQuote.vue';
    import Header from './components/Header.vue';

    export default {
        data() {
            return {
                quotes: [
                    'Just a Quote to see something'
                ],
                maxQuotes: 10,
            };
        },
        components: {
            appQuoteGrid: QuoteGrid,
            appNewQuote: NewQuote,
            appHeader: Header,
        },
        methods: {
            addQuote(quote) {
                if (this.quotes.length >= this.maxQuotes) {
                    return alert('Please delete quotes first!');
                }
                this.quotes.push(quote);
            },
            deleteQuote(i) {
                this.quotes.splice(i, 1);
            },
        },
    }
</script>

<style>
</style>
