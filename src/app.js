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
        newEventState: {
            isOkay: true,
            message: ""
        },
        events: []
    },

    created: function() {
        this.fetchEvents();
    },

    methods: {

        resetEvents: function(newEvents) {
            this.events = newEvents || [];
        },

        fetchEvents: function() {
            this.$http
                .get(API_EVENTS_URL)
                .then((response) => {
                    console.log(response);
                    return response.json();
                }, (response) => {
                    // Error
                    console.log(response)
                })
                .then(this.resetEvents);
        },

        addEvent: function(newEvent) {
            console.log("foo");
            if (newEvent) {
                this.events.push(newEvent);
            }
        },

        fail: function(message) {
            this.newEventState.isOkay = false;
            this.newEventState.message = message;
        },

        submitNewEvent: function() {
            if (this.event.name) {
                var newEvent = this.event;
                this.$http
                    .post(API_EVENTS_URL, newEvent)
                    .then((response) => {
                        console.log(response);
                        this.event = {
                            name: "",
                            description: "",
                            console: ""
                        };
                        this.newEventState.isOkay = true;
                        return response.json();
                    }, (response) => {
                        console.log(response);
                        this.fail("The server responded with status code " + response.status +
                                  ". Please resubmit your event again in couple minutes.");
                    })
                    .then(this.addEvent);
            }
            else {
                this.fail("The event is missing a name. Please provide a name and " +
                          "then resubmit.");
            }
        },

        deleteEvent: function(ix) {
            this.events.splice(ix, 1);
        }
    }
});
