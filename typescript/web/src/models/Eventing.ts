type Callback = () => void;

// Eventing contains the event logic for a given model.
export class Eventing {
  // the [key: string] annotation lets TS know that the object's keys will be strings,
  // but that we don't know what their names will be.
  events: { [key: string]: Callback[] } = {};

  on = (eventName: string, callback: Callback): void => {
    const handlers = this.events[eventName] || [];
    handlers.push(callback);
    this.events[eventName] = handlers;
  }

  trigger = (eventName: string): void => {
    const handlers = this.events[eventName];
    if (!handlers || handlers.length === 0) return;
    handlers.forEach(callback => callback());
  }
}
