var CartView = Backbone.View.extend({
  template: App.templates.cart,
  el: $('#cart').get(0),
  events: {
    'click a': 'destroy'
  },

  destroy (e) {
    e.preventDefault();

    var $e = $(e.target);
    this.collection.trigger('destroy', Number($e.attr('data-id')));
    this.render();
  },

  render () {
    this.$el.html(this.template({
      quantity: this.collection.getQuantity(),
      items: this.collection.toJSON(),
      total: this.collection.getTotal()
    }));
  },

  initialize () {
    this.render();
    this.listenTo(this.collection, 'cart_updated', this.render);
  }
});
