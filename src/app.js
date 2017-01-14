import Vue from 'vue'
import VueResource from 'vue-resource'

Vue.use(VueResource);

const API_URL = "/api";
const API_EVENTS_URL = API_URL + "/events";

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
        this.fetchEvents();
    },

    methods: {

        fetchEvents: function() {
            console.log("Requesting data");
            this.$http
                .get(API_EVENTS_URL)
                .then((data) => {
                    console.log(data);
                    for (var event of data.body) {
                        console.log("Pushing an event");
                        this.events.push(event);
                    }
                }, (err) => {
                    console.log(err)
                });
        },

        addEvent: function() {
            if (this.event.name) {
                this.events.push(this.event);
                this.event = {
                    name: "",
                    description: "",
                    console: ""
                }
            }
        },

        deleteEvent: function(ix) {
            this.events.splice(ix, 1);
        }
    }
});
