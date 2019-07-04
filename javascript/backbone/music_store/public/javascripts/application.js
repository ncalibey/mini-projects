var App = {
  templates: JST,
  $el: $('main'),

  indexView () {
    this.index = new IndexView();
    this.renderAlbums();
    this.createCart();
    this.bindEvents();
  },

  renderAlbums () {
    this.albums.each(this.renderAlbumView);
  },

  createCart () {
    this.cart = new CartItems();
    this.cart.view = new CartView({
      collection: this.cart
    });
  },

  renderAlbumView (album) {
    new AlbumView({
      model: album
    });
  },

  newAlbum () {
    new NewAlbumView();
  },

  bindEvents () {
    _.extend(this, Backbone.Events);
    this.listenTo(this.index, 'add_album', this.newAlbum);
    this.on('addToCart', this.cart.addItem.bind(this.cart));
  },
};


Handlebars.registerHelper('formatPrice', price => {
  return Number(price).toFixed(2);
});
