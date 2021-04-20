function infiniteScroll(url, trigger) {
  return {
    triggerElement: null,
    page: 0,
    observer: null,
    items: [],
    init(elementId) {
      const ctx = this
      this.triggerElement = document.querySelector(elementId ? elementId : trigger)
      this.observer = new IntersectionObserver(function(entries) {
        if(entries[0].isIntersecting === true) {
          ctx.loadMore()
        }
      }, { threshold: [0] })
      this.observer.observe(this.triggerElement)
    },
    loadMore() {
      fetch(url + this.page).then(response => response.json()).then(data => {
        if(data.length > 0) {
          this.items = this.items.concat(data)
          this.page++
        } else {
          this.observer.unobserve(this.triggerElement)
          this.triggerElement.parentNode.removeChild(this.triggerElement)
        }
      });
    }
  }
}