// TODO: // Generated by CoffeeScript 1.12.4
// TODO: (function() {
// TODO:   (require("chai")).should();
// TODO: 
// TODO:   describe("map reducer", function() {
// TODO:     var Actions, R;
// TODO:     R = require("../../src/reducers/map");
// TODO:     Actions = require("../../src/Actions");
// TODO:     it("should be a function", function() {
// TODO:       return R.should.be.a("function");
// TODO:     });
// TODO:     describe("marker", function() {
// TODO:       return it("should be set when the action was fired", function() {
// TODO:         var action, actionToBeDispatched, marker;
// TODO:         actionToBeDispatched = Actions.setMarker({
// TODO:           lat: 5,
// TODO:           lng: 7
// TODO:         });
// TODO:         action = null;
// TODO:         actionToBeDispatched((function(arg) {
// TODO:           return action = arg;
// TODO:         }), (function() {
// TODO:           return {};
// TODO:         }));
// TODO:         marker = R({}, action).marker;
// TODO:         marker.lat.should.equal(5);
// TODO:         return marker.lng.should.equal(7);
// TODO:       });
// TODO:     });
// TODO:     return describe("bounding box", function() {
// TODO:       return it("should be set when the action was fired", function() {
// TODO:         var action, bbox;
// TODO:         action = Actions.setBbox({
// TODO:           foo: "bar"
// TODO:         });
// TODO:         bbox = R({}, action).bbox;
// TODO:         return bbox.foo.should.equal("bar");
// TODO:       });
// TODO:     });
// TODO:   });
// TODO: 
// TODO: }).call(this);