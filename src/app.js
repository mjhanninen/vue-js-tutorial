import Vue from 'vue'

console.log("Hitting app.js");

export var app = new Vue({
    el: "#events",
    data: {
        event: {
            name: "",
            description: "",
            date: ""
        },
        events: []
    },
    created: function() {
        console.log("app created fired");
        this.fetchEvents();
    },
    ready: function() {
        console.log("app ready fired");
        this.fetchEvents();
    },
    methods: {
        fetchEvents: function() {
            console.log("fetchEvents fired");
            var events = [
                {
                    id: 1,
                    name: "TIFF",
                    description: "Toronto International Film Festival",
                    date: "2015-09-10"
                },
                {
                    id: 2,
                    name: "The Martian Premiere",
                    description: "The Martian comes to theaters",
                    date: "2015-10-02"
                }
            ];
            for (var e of events) {
                this.events.push(e);
            }
        },
        addEvents: function() {
            if (this.event.name) {
                this.events.push(this.event);
                this.event = {
                    name: "",
                    description: "",
                    console: ""
                }
            }
        }
    }
});

console.log("Finished app.js");
