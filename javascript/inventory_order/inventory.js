var inventory;

(function () {
  function cacheTemplate() {
    var $invTemplate = $('#inventory_item').remove();

    return Handlebars.compile($invTemplate.html());
  }

  var template = cacheTemplate();
  var $inventory = $('#inventory');

  inventory = {
    collection: [],
    lastId: 0,
    setDate: function () {
      var date = new Date();

      $('#order_date').text(date.toUTCString());
    },
    addItem: function () {
      this.lastId += 1;
      this.collection.push({
        id: this.lastId,
        name: '',
        stockNumber: '',
        quantity: 1,
      });
    },
    appendItem: function () {
      var collection = this.collection;

      $inventory.append(template(collection[collection.length - 1]));
    },
    newItem: function (e) {
      e.preventDefault();

      this.addItem();
      this.appendItem();
    },
    findPropertyType: function ($item) {
      var type = $item.attr('name').replace(/.*(name|quantity|stock).*/g, '$1');

      if (type === 'stock') type = 'stockNumber';

      return type;
    },
    findRowNumber: function ($item) {
      return $item.closest('tr').index();
    },
    update: function (e) {
      var $input = $(e.target);
      var propType = this.findPropertyType($input);
      var rowNumber = this.findRowNumber($input);
      var item = this.collection[rowNumber];

      item[propType] = $input.val();
    },
    updateItem: function (e) {
      this.update(e);
    },
    deleteItem: function (e) {
      e.preventDefault();

      var $row = $(e.target).closest('tr');
      var rowNumber = this.findRowNumber($row);

      this.collection.splice(rowNumber, 1);
      $row.remove();
    },
    bindEvents: function () {
      $('#add_item').on('click', this.newItem.bind(this));
      $inventory.on('blur', 'input', this.updateItem.bind(this));
      $inventory.on('click', 'a', this.deleteItem.bind(this));
    },
    init: function () {
      this.setDate();
      this.bindEvents();
    }
  };
})();

$(inventory.init.bind(inventory));
