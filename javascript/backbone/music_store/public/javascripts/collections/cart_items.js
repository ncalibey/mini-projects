var CartItems = Backbone.Collection.extend({
  setTotal () {
    this.total = this.toJSON().reduce((a, b) => {
      return a + b.price * b.quantity
    }, 0);

    return this;
  },

  getTotal () {
    return this.total;
  },

  setQuantity () {
    this.quantity = this.toJSON().reduce((a, b) => {
      return a + b.quantity
    }, 0);

    return this;
  },

  getQuantity () {
    return this.quantity;
  },

  readStorage () {
    var storedCart = JSON.parse(localStorage.getItem('cart'));
    this.reset(storedCart);
    this.setTotal().setQuantity();
  },

  updateStorage () {
    localStorage.setItem('cart', JSON.stringify(this.toJSON()));
  },

  addItem (item) {
    var existing = this.get(item.get('id'));

    if (existing) {
      existing.set('quantity', existing.get('quantity') + 1);
    } else {
      existing = item.clone();
      existing.set('quantity', 1);
      this.add(existing);
    }

    this.update();
    this.trigger('cart_updated');
  },

  destroy (id) {
    this.remove(id);
    this.update();
  },

  update () {
    this.setTotal().setQuantity().updateStorage();
  },

  initialize () {
    this.readStorage();
    this.on('destroy', this.destroy);
  }
});
