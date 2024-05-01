function load_places_map(cord) {
    ymaps.ready(init);
    let v = [];
    let list = cord.split(", ");
    v.push(list[0]);
    v.push(list[1]);
    function init() {
      console.log(cord);
      console.log(v);
      var suggestView = new ymaps.SuggestView('suggest1'),
        suggestView = new ymaps.SuggestView('suggest2'),
        map, routePanelControl,
        addrFrom, addrTo;
    
      var map; 
      map = new ymaps.Map('map', {
        center: v,
        zoom: 14, 
        controls: ["typeSelector", "fullscreenControl", "zoomControl", "geolocationControl"]
      });
    
      map.setType('yandex#satellite');
          map.geoObjects
              .add(new ymaps.Placemark(v, {
              balloonContent: '',
              iconCaption: ''
          }, {  
              preset: 'islands#blueCircleDotIconWithCaption',
              iconCaptionMaxWidth: '50',
              iconLayout: 'default#image',
              iconImageHref: '/static/images/myIcon.gif',
              icon_imagesize: [30, 42],
              iconImageOffset: [-3, -42]
        }));
    
      //////////////////////////////////
      routePanelControl = new ymaps.control.RoutePanel({
          options: {
            showHeader: true,
            title: 'Расчёт расстояния'
          }
        });
        var zoomControl = new ymaps.control.ZoomControl({
          options: {
            size: 'small',
            float: 'none',
            position: {
              bottom: 145,
              right: 10
            }
          }
        });
        routePanelControl.routePanel.options.set({
          types: {
            auto: true
          }
        });
        routePanelControl.routePanel.state.set({
          fromEnabled: false,
          toEnabled: false
        });
    
        map.controls.add(routePanelControl).add(zoomControl);
      //////////////////////////////////
    
      function create_path() {
        addrFrom = document.body.querySelector("#suggest1");
        addrTo = document.body.querySelector("#suggest2");
        
        geocode('#suggest1');
        geocode('#suggest2');
        showRoute(addrFrom.value, addrTo.value);
      }
    
      on('body', 'click', '#button3', function() { 
        addrFrom = document.body.querySelector("#suggest1");
        addrTo = document.body.querySelector("#suggest2");
        addrFrom.style.setProperty('border', 'inherit', 'important');
        addrTo.style.setProperty('border', 'inherit', 'important');
        if (!addrFrom.value) {
          addrFrom.style.setProperty('border', '1px #FF0000 solid', 'important');
          toast_error("Укажите адрес назначения");
          return
        }
        else if (!addrTo.value) {
          addrTo.style.setProperty('border', '1px #FF0000 solid', 'important');
          toast_error("Укажите адрес назначения");
          return
        }
        create_path();
      });
    
      on('body', 'click', '.get_ma', function() { 
        geocode('#suggest1');
        geocode('#suggest2');
        addrFrom = document.body.querySelector("#suggest1").value;
        addrTo = document.body.querySelector("#suggest2").value;
        if (addrFrom && addrTo) {
          showRoute(addrFrom, addrTo);
        } else {
          alert("Согласитесь на использование Вашего местоположение");
        }
      });
    
    
      function geocode(ctrl_id) {
        document.body.querySelector(ctrl_id)
        var request = document.body.querySelector(ctrl_id).value;
        ymaps.geocode(request).then(function(res) {
          var obj = res.geoObjects.get(0),
            error, hint;
    
          if (obj) {
            switch (obj.properties.get('metaDataProperty.GeocoderMetaData.precision')) {
              case 'exact':
                break;
              case 'number':
              case 'near':
              case 'range':
                error = '';
                hint = 'Уточните номер дома';
                break;
              case 'street':
                error = '';
                hint = 'Уточните номер дома';
                break;
              case 'other':
              default:
                error = '';
                hint = 'Уточните адрес';
            }
          } else {
            error = '';
            hint = 'Уточните адрес';
          }
    
          if (error) {
            if (ctrl_id == '#suggest1') {
              addrFrom = null
            } else {
              addrTo = null
            }
            showError(ctrl_id, error);
            showMessage(ctrl_id, hint);
          } else {
            if (ctrl_id == '#suggest1') {
              addrFrom = obj
            } else {
              addrTo = obj
            }
            showResult(ctrl_id);
          }
          if (addrFrom && addrTo) {
            document.body.querySelector("#notice3").style.display = "none";
          } else {
            document.body.querySelector("#notice3").style.display = "block";
          }
        }, function(e) {
          console.log(e)
        })
    
      }
    
      function showResult(ctrl_id) {
        document.body.querySelector(ctrl_id).classList.remove("input_error");
        document.body.querySelector("#notice1").style.display = "none";
        document.body.querySelector("#notice2").style.display = "none";
        if (ctrl_id == '#suggest1') {
          showMessage(ctrl_id, addrFrom.getAddressLine());
        } else {
          showMessage(ctrl_id, addrTo.getAddressLine());
        }
      }
    
      function showError(ctrl_id, message) {
        document.body.querySelector(ctrl_id).classList.add("input_error");
        if (ctrl_id == '#suggest1') {
          document.body.querySelector("#notice1").innerHTML = message;
          document.body.querySelector("#notice1").style.display = "block";
        } else {
          document.body.querySelector("#notice2").innerHTML = message;
          document.body.querySelector("#notice2").style.display = "block";
        }
    
      }
    
      function showRoute(from, to) {
        routePanelControl.routePanel.state.set({
          from: from,
          to: to
        });
        routePanelControl.routePanel.getRouteAsync().then(function(route) {
          route.model.setParams({
            results: 1
          }, true);
          route.model.events.add('requestsuccess', function() {
            var activeRoute = route.getActiveRoute();
            if (activeRoute) {
              var length = route.getActiveRoute().properties.get("distance");
                balloonContentLayout = ymaps.templateLayoutFactory.createClass(
                  '<span>Расстояние: ' + length.text + '.</span>');
                
                balloonContentLayout = ymaps.templateLayoutFactory.createClass(
                  '<span>Расстояние: ' + length.text + '.</span>');
              route.options.set('routeBalloonContentLayout', balloonContentLayout);
              activeRoute.balloon.open();
            }
          });
        });
      }
    
      function showMessage(ctrl_id, message) {
        if (ctrl_id == '#suggest1') {
          //document.body.querySelector("#messageHeader1").innerHTML = '';
          document.body.message1("#messageHeader1").innerHTML = message;
        } else {
          //document.body.querySelector("#messageHeader2").innerHTML = '';
          document.body.message1("#message2").innerHTML = message;
        }
      }
      create_path();
    }
  }; 

  function load_deceadeds_map(cord) {
    ymaps.ready(init); 
    let v = [];
    list = cord.split(",");
    
    console.log(list[0]);
    console.log(list[1]);
    v.push(list[0]);
    v.push(list[1]);
    function init() {
      var suggestView = new ymaps.SuggestView('suggest1'),
        suggestView = new ymaps.SuggestView('suggest2'),
        map, routePanelControl,
        addrFrom, addrTo;
    
      var map; 
      map = new ymaps.Map('map', {
        center: v,
        zoom: 18,
        controls: ["typeSelector", "fullscreenControl", "zoomControl", "geolocationControl"]
      });
    
      map.setType('yandex#satellite');
          map.geoObjects
              .add(new ymaps.Placemark(v, {
              balloonContent: '',
              iconCaption: ''
          }, {
              preset: 'islands#blueCircleDotIconWithCaption',
              iconCaptionMaxWidth: '50',
              iconLayout: 'default#image',
              iconImageHref: '/static/images/myIcon.gif',
              icon_imagesize: [30, 42],
              iconImageOffset: [-3, -42]
          }));
    
      //////////////////////////////////
      routePanelControl = new ymaps.control.RoutePanel({
          options: {
            showHeader: true,
            title: 'Расчёт расстояния'
          }
        });
        var zoomControl = new ymaps.control.ZoomControl({
          options: {
            size: 'small',
            float: 'none',
            position: {
              bottom: 145,
              right: 10
            }
          }
        });
        routePanelControl.routePanel.options.set({
          types: {
            auto: true
          }
        });
        routePanelControl.routePanel.state.set({
          fromEnabled: false,
          toEnabled: false
        });
    
        map.controls.add(routePanelControl).add(zoomControl);
      //////////////////////////////////
    
      function create_path() {
        addrFrom = document.body.querySelector("#suggest1");
        addrTo = document.body.querySelector("#suggest2");
        
        geocode('#suggest1');
        geocode('#suggest2');
        showRoute(addrFrom.value, addrTo.value);
      }
    
      on('body', 'click', '#button3', function() { 
        addrFrom = document.body.querySelector("#suggest1");
        addrTo = document.body.querySelector("#suggest2");
        addrFrom.style.setProperty('border', 'inherit', 'important');
        addrTo.style.setProperty('border', 'inherit', 'important');
        if (!addrFrom.value) {
          addrFrom.style.setProperty('border', '1px #FF0000 solid', 'important');
          toast_error("Укажите адрес назначения");
          return
        }
        else if (!addrTo.value) {
          addrTo.style.setProperty('border', '1px #FF0000 solid', 'important');
          toast_error("Укажите адрес назначения");
          return
        }
        create_path();
      });
    
      on('body', 'click', '.get_ma', function() { 
        geocode('#suggest1');
        geocode('#suggest2');
        addrFrom = document.body.querySelector("#suggest1").value;
        addrTo = document.body.querySelector("#suggest2").value;
        if (addrFrom && addrTo) {
          showRoute(addrFrom, addrTo);
        } else {
          alert("Согласитесь на использование Вашего местоположение");
        }
      });
    
    
      function geocode(ctrl_id) {
        document.body.querySelector(ctrl_id)
        var request = document.body.querySelector(ctrl_id).value;
        ymaps.geocode(request).then(function(res) {
          var obj = res.geoObjects.get(0),
            error, hint;
    
          if (obj) {
            switch (obj.properties.get('metaDataProperty.GeocoderMetaData.precision')) {
              case 'exact':
                break;
              case 'number':
              case 'near':
              case 'range':
                error = '';
                hint = 'Уточните номер дома';
                break;
              case 'street':
                error = '';
                hint = 'Уточните номер дома';
                break;
              case 'other':
              default:
                error = '';
                hint = 'Уточните адрес';
            }
          } else {
            error = '';
            hint = 'Уточните адрес';
          }
    
          if (error) {
            if (ctrl_id == '#suggest1') {
              addrFrom = null
            } else {
              addrTo = null
            }
            showError(ctrl_id, error);
            showMessage(ctrl_id, hint);
          } else {
            if (ctrl_id == '#suggest1') {
              addrFrom = obj
            } else {
              addrTo = obj
            }
            showResult(ctrl_id);
          }
          if (addrFrom && addrTo) {
            document.body.querySelector("#notice3").style.display = "none";
          } else {
            document.body.querySelector("#notice3").style.display = "block";
          }
        }, function(e) {
          console.log(e)
        })
    
      }
    
      function showResult(ctrl_id) {
        document.body.querySelector(ctrl_id).classList.remove("input_error");
        document.body.querySelector("#notice1").style.display = "none";
        document.body.querySelector("#notice2").style.display = "none";
        if (ctrl_id == '#suggest1') {
          showMessage(ctrl_id, addrFrom.getAddressLine());
        } else {
          showMessage(ctrl_id, addrTo.getAddressLine());
        }
      }
    
      function showError(ctrl_id, message) {
        document.body.querySelector(ctrl_id).classList.add("input_error");
        if (ctrl_id == '#suggest1') {
          document.body.querySelector("#notice1").innerHTML = message;
          document.body.querySelector("#notice1").style.display = "block";
        } else {
          document.body.querySelector("#notice2").innerHTML = message;
          document.body.querySelector("#notice2").style.display = "block";
        }
    
      }
    
      function showRoute(from, to) {
        routePanelControl.routePanel.state.set({
          from: from,
          to: to
        });
        routePanelControl.routePanel.getRouteAsync().then(function(route) {
          route.model.setParams({
            results: 1
          }, true);
          route.model.events.add('requestsuccess', function() {
            var activeRoute = route.getActiveRoute();
            if (activeRoute) {
              var length = route.getActiveRoute().properties.get("distance");
                balloonContentLayout = ymaps.templateLayoutFactory.createClass(
                  '<span>Расстояние: ' + length.text + '.</span>');
                
                balloonContentLayout = ymaps.templateLayoutFactory.createClass(
                  '<span>Расстояние: ' + length.text + '.</span>');
              route.options.set('routeBalloonContentLayout', balloonContentLayout);
              activeRoute.balloon.open();
            }
          });
        });
      }
    
      function showMessage(ctrl_id, message) {
        if (ctrl_id == '#suggest1') {
          //document.body.querySelector("#messageHeader1").innerHTML = '';
          document.body.message1("#messageHeader1").innerHTML = message;
        } else {
          //document.body.querySelector("#messageHeader2").innerHTML = '';
          document.body.message1("#message2").innerHTML = message;
        }
      }
      create_path();
    }
  }; 