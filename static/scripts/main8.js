function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function getCookie(name) {
  const cookies = document.cookie.split(';');
  for (let i = 0; i < cookies.length; i++) {
      let c = cookies[i].trim().split('=');
      if (c[0] === name) {
          return c[1];
      }
  }
  return "";
}
function setCookie(name, value, days) {
  let cookie = `${name}=${encodeURIComponent(value)}`;
  if (days) {
      const expiry = new Date();
      expiry.setDate(expiry.getDate() + days);
      cookie += `; expires=${expiry.toUTCString()}`;
  }
  document.cookie = cookie + "; path=/";
};
function eraseCookie(name) {   
  document.cookie = name+'=; Max-Age=-99999999;';  
}
function deleteAllCookies() {
  const cookies = document.cookie.split(";");
  setCookie("userrr", 0, 90);
  for (let i = 0; i < cookies.length; i++) {
      const cookie = cookies[i];
      const eqPos = cookie.indexOf("=");
      const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
      document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
  }
}
on('body', 'click', '.logout_hundler', function() {
  window.location.href = "/";
  setCookie("userrr", 0, 90);
  eraseCookie("userrr");
  window.location.href = "/";
  eraseCookie("userrr");
  window.location.href = "/";
})

class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('body');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.style.display = 'none';
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.style.display = 'none';
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
}

function toast_success(text) {
    var toasts = new ToastManager();
    toasts.showSuccess(text)
}

function toast_error(text) {
    var toasts = new ToastManager();
    toasts.showError(text)
}

function toast_info(text) {
    var toasts = new ToastManager();
    toasts.showInfo(text)
}

function toast_warning(text) {
    var toasts = new ToastManager();
    toasts.showWarning(text)
}

function elementInViewport(el){var bounds = el.getBoundingClientRect();return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));}


window.onscroll = function() {
  if (window.pageYOffset > 100) {
    document.body.classList.add("sticky-header")
  }
  else {
    document.body.classList.remove("sticky-header")
  }
};

on('body', 'click', '#logg', function() {
    _this = this;
    form = _this.parentElement.parentElement;
    response = form.querySelector(".api_response");
  
    if (!form.querySelector("#id_username").value){
      form.querySelector("#id_username").style.border = "1px #FF0000 solid";
      response.innerHTML = "Введите логин!";
      response.classList.add("error");
      return
    }
    else if (!form.querySelector("#id_password").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      response.innerHTML = "Введите пароль!";
      response.classList.add("error")
      return
    }
    else {
      _this.disabled = true;
    }
  
    form_data = new FormData(form);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/login/", true );
    //link.setRequestHeader('Content-Type', 'application/x-www-form-urlencoded');
  
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      setCookie("userrr", link.responseText, 300);
      window.location.href = "/"
      }
    else {
      _this.disabled = false;
      response.style.display = "block";
      //response.innerHTML = "Логин или пароль - неверный!";
      response.classList.add("error");
      form.querySelector("#id_username").style.display = "block";
      form.querySelector("#id_username").value = '';
      form.querySelector("#id_password").value = '';
    }};
    link.send(form_data);
  });

on('body', 'click', '.login_window', function() {
  console.log(document.body.querySelector("#modal_container"));
  document.body.querySelector("#modal_container").innerHTML = '<div tabindex="-1" style="position: relative; z-index: 1050; display: block;"><div class="modal fade show" role="dialog" tabindex="-1" style="display: block;"><div class="modal-dialog" role="document"><div class="modal-content"><div class="modal-header"><h5 class="modal-title"></h5><button type="button" class="close modal_close" aria-label="Close"><span aria-hidden="true">×</span></button></div><div class="modal-body"><div class="always-float"><form class="my-5"><h3 class="text-white text-center mb-3">Авторизация</h3><div class="form-group"><input id="id_username" name="username" type="text" class="form-input" /><label for="id_username" class="">Логин</label><span class="required-mark">*</span></div><div class="form-group"><input id="id_password" name="password" type="password" class="form-input" /><label for="id_password" class="">Пароль</label><span class="required-mark">*</span></div><p class="api_response"></p><button type="button" class="py-2 mb-3 w-100 btn btn-primary" id="logg"><i class="icon-user"></i><span>Вход</span></button></form></div></div></div></div></div><div class="modal-backdrop fade show"></div></div>';
});

on('body', 'click', '.signup_window', function() {
  document.body.querySelector("#modal_container").innerHTML = '<div tabindex="-1" style="position: relative; z-index: 1050; display: block;"><div class="modal fade show" role="dialog" tabindex="-1" style="display: block;"><div class="modal-dialog" role="document"><div class="modal-content"><div class="modal-header"><button type="button" class="close" aria-label="Close"><span aria-hidden="true">×</span></button></div><div class="modal-body"><div class="always-float"><form class="my-md-5"><h3 class="text-white text-center mb-3">Регистрация</h3><p class="api_response"></p><div class="form-group"><input name="username" id="id_username" placeholder="Логин" type="text" class="form-control"><label for="id_username">Логин</label><span class="required-mark">*</span></div><div class="form-group"><input name="first_name" id="id_first_name" placeholder="Ваше имя" type="text" class="form-control"><label for="id_first_name">Ваше имя</label><span class="required-mark">*</span></div><div class="form-group"><input name="last_name" id="id_last_name" placeholder="Ваша фамилия" type="text" class="form-control"><label for="id_last_name">Ваша фамилия</label><span class="required-mark">*</span></div><div class="form-group"><input name="phone" id="id_phone" placeholder="Телефон" type="text" class="form-control"><label for="id_phone">Телефон</label><span class="required-mark">*</span></div><div class="form-group"><input name="email" id="id_email" placeholder="Почта" type="email" class="form-control"><label for="id_email">Почта</label><span class="required-mark">*</span></div><div class="form-group"><input name="password" id="id_password" placeholder="Пароль" type="password" class="form-control"><label for="id_password">Повторите Пароль</label><span class="required-mark">*</span></div><div class="form-group"><input name="password2" id="id_password2" placeholder="Пароль" type="password" class="form-control"><label for="id_password2">Повторите Пароль</label><span class="required-mark">*</span></div><div class="mb-3 form-check"><label for="agreement" class="form-check-label"><input name="agreement" id="id_agreement" type="checkbox" class="form-check-input">Согласен  <a href="/terms/" target="_blank">Пользовательским соглашением,</a> и <br><a href="/policy" target="_blank">Политикой конфиденциальности и обработки персональных данных.</a></label></div><button type="button" class="py-2 mb-3 w-100 btn btn-primary" id="signup"><i class="icon-arrow-right"></i><span>Зарегистрироваться</span></button></form></div></div></div></div></div><div class="modal-backdrop fade show"></div></div>';
});

function close_model() {
  document.body.querySelector("#modal_container").innerHTML = "";
};

on('body', 'click', '.modal-backdrop', function() {
  close_model()
});
on('body', 'click', '.close_model', function() {
  close_model()
});

on('body', 'click', '.search_sorted', function() {
  this.parentElement.previousElementSibling.querySelector("span").innerHTML = this.getAttribute("data-icon");
  this.parentElement.parentElement.parentElement.querySelector("._filter").value = this.getAttribute("data-type");
  list = this.parentElement.querySelectorAll(".search_sorted");
  for (var i = 0, len = list.length; i < len; i++) {
    list[i].classList.remove("active");
  }
  this.classList.add("active");
});

  
on('body', 'click', '#signup', function() {
    _this = this;
    form = _this.parentElement.parentElement;
    response = form.querySelector(".api_response");
    console.log(form.querySelector("#id_username"));
    if (!form.querySelector("#id_username").value){
      form.querySelector("#id_username").style.border = "1px #FF0000 solid";
      response.innerHTML = "Логин - обязательное поле!";
      response.classList.add("error");
      return
    } else if (!form.querySelector("#id_first_name").value){
      form.querySelector("#id_first_name").style.border = "1px #FF0000 solid";
      response.innerHTML = "Ваше имя - обязательное поле!";
      response.classList.add("error");
      return
    } else if (!form.querySelector("#id_last_name").value){
      form.querySelector("#id_last_name").style.border = "1px #FF0000 solid";
      response.innerHTML = "Ваша фамилия - обязательное поле!";
      response.classList.add("error");
      return
    } else if (!form.querySelector("#id_password").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      toast_error("Пароль - обязательное поле!");
      return
    }
    else if (form.querySelector("#id_password").value != form.querySelector("#id_password2").value){
      form.querySelector("#id_password").style.border = "1px #FF0000 solid";
      form.querySelector("#id_password2").style.border = "1px #FF0000 solid";
      toast_error("Пароль - обязательное поле!");
      return
    }
    else {
      form.querySelector("#id_password").style.border = "unset";
      form.querySelector("#id_password2").style.border = "unset";
      form.querySelector("#id_username").style.border = "unset";
      form.querySelector("#id_first_name").style.border = "unset";
      form.querySelector("#id_last_name").style.border = "unset";
      this.disabled = true;
      response.classList.remove("error");
    }

    form.querySelector("#signup").setAttribute("disabled", "true");
  
    form_data = new FormData(form);
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/signup/", true );
  
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      setCookie("userrr", link.responseText, 300);
      window.location.href = "/"
      }
    else {
      _this.disabled = false;
      response.style.display = "block";
      response.innerHTML = "not ok";
      response.classList.add("error");
    }};
    link.send(form_data);
  });

 
on('body', 'click', '.search_deceaseds', function() {
  form = this.parentElement.parentElement.parentElement.parentElement.parentElement;
 
  block = form.nextElementSibling; 
  block.innerHTML = "";
  if (
    !form.querySelector("#id_last_name").value
    && !form.querySelector("#id_first_name").value
    && !form.querySelector("#id_middle_name").value
    && !form.querySelector("#id_birth_date").value
    && !form.querySelector("#id_death_date").value
    && !form.querySelector("#id_place").value
    && !form.querySelector("#is_veteran").checked
    && !form.querySelector("#is_famous").checked
    && !form.querySelector("#with_photo").checked
    && !form.querySelector("#with_coordinates").checked
  ) 
  {
    toast_info("Заполните хотя бы одно поле формы");
    return 
  } 

  url = "/main_search?qw=";
  if (form.querySelector("#id_first_name").value) {
    url += "&first_name=" + form.querySelector("#id_first_name").value
  };
  if (form.querySelector("#id_last_name").value) {
    url += "&last_name=" + form.querySelector("#id_last_name").value
  };
  if (form.querySelector("#id_middle_name").value) {
    url += "&middle_name=" + form.querySelector("#id_middle_name").value
  };
  if (form.querySelector("#id_birth_date").value) {
    url += "&birth_date=" + form.querySelector("#id_birth_date").value
  };
  if (form.querySelector("#id_death_date").value) {
    url += "&death_date=" + form.querySelector("#id_death_date").value
  };
  if (form.querySelector("#id_place") && form.querySelector("#id_place").value) {
    url += "&place=" + form.querySelector("#id_place").value
  }; 
  if (form.querySelector("#is_veteran").checked) {
    url += "&is_veteran=true"
  }; 
  if (form.querySelector("#is_famous").checked) {
    url += "&is_famous=true"
  };
  if (form.querySelector("#with_photo") && form.querySelector("#with_photo").checked) {
    url += "&with_photo=true"
  };
  if (form.querySelector("#with_coordinates") && form.querySelector("#with_coordinates").checked) {
    url += "&with_coordinates=true"
  };

  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
        console.log("success!");
        block.innerHTML = link.responseText;
    } else { console.log("status", this.status);  };
  };
  link.send( null );
});


on('body', 'click', '.search_organizations', function() {
  form = this.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_name").style.setProperty('border', '1px solid #ced4da', 'important');
 
  block = form.nextElementSibling;
  block.innerHTML = "";
  if (!form.querySelector("#id_name").value) {
    toast_info("Название фирмы - обязательное поле");
    form.querySelector("#id_name").style.border = "1px #FF0000 solid";
    return 
  }

  url = "/org_search?name=" + form.querySelector("#id_name").value;
  if (form.getAttribute("service-pk")) {
    url += "&service=" + form.getAttribute("service-pk")
  };
  if (form.querySelector("#id_location") && form.querySelector("#id_location").value.length > 2) {
    url += "&location=" + form.querySelector("#id_location").value.replace(/\s+/g, ' ')
  }; 

  var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
    if ( this.readyState == 4 && this.status == 200 ) {
        console.log("success!");
        block.innerHTML = link.responseText;
    } else { console.log("status", this.status);  };
  };
  link.send( null );
});


on('body', 'change', '.load_region_items', function() {
  var val = this.value; 
  block = this.parentElement.nextElementSibling;
  if (block.classList.contains("no_region_items")) {
    return;
  }
  if (val == '') {
    block.innerHTML = "";
  } else {
    var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'GET', "/load_region_geo_items/" + val + "/", true );
    link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
    link.onreadystatechange = function () {
      if ( link.readyState == 4 ) { 
          if ( link.status == 200 ) {
              block.innerHTML = link.responseText;
          }
      }
  };
  link.send( null );
  };
});

on('body', 'change', '.load_regions', function() {
  var val = this.value; 
  block = this.parentElement.nextElementSibling;
  if (!block.classList.contains("load_items")) {
    return;
  }
  if (val == '') {
    block.innerHTML = "";
  } else {
      var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', "/load_regions/" + val + "/", true );
      link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      link.onreadystatechange = function () {
        if ( link.readyState == 4 ) { 
            if ( link.status == 200 ) {
                block.innerHTML = link.responseText;
            }
        }
    };
    link.send( null );
  };
});

on('body', 'input', '.place_search', function() {
  console.log("place_search change");
  _this = this; 
  var val = _this.value;
  block = _this.nextElementSibling;
  console.log("val ", val);
  if (val == '') {
    _this.value = "";
    block.classList.add("hidden");
    _this.previousElementSibling.value = "";
  } else {
      var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', "/search_places/?name=" + val, true );
      link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      link.onreadystatechange = function () {
        if ( link.readyState == 4 ) { 
            if ( link.status == 200 ) {
              _this.nextElementSibling.innerHTML = link.responseText;
              if (link.responseText.length > 10) {
                block.classList.remove("hidden");
              }
            } 
        }
    };
    link.send( null );
  };
});
on('body', 'input', '.braves_search', function() {
  console.log("brave_search change");
  _this = this; 
  var val = _this.value;
  block = _this.nextElementSibling;
  console.log("val ", val);
  if (val == '') {
    _this.value = "";
    block.classList.add("hidden");
    _this.previousElementSibling.value = "";
  } else {
      var link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      link.open( 'GET', "/search_braves/?name=" + val, true );
      link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      link.onreadystatechange = function () {
        if ( link.readyState == 4 ) { 
            if ( link.status == 200 ) {
              _this.nextElementSibling.innerHTML = link.responseText;
              if (link.responseText.length > 10) {
                block.classList.remove("hidden");
              }
            } 
        }
    };
    link.send( null );
  };
});
on('body', 'click', '.place_search_item', function() {
  _value = this.parentElement.previousElementSibling;
  _value.value = this.innerHTML;
  _value.previousElementSibling.value = this.getAttribute("data-pk");
  this.parentElement.classList.add("hidden");
});
 
on('body', 'click', '.place_click', function() {
  block = this.nextElementSibling;
  if (block.classList.contains("hidden")) {
    block.classList.remove("hidden")
  } 
  else {
    block.classList.add("hidden");
  }
});

on('body', 'click', '.show_dop_search_form', function() {
  block = this.parentElement.parentElement.nextElementSibling;
  if (block.classList.contains("hidden")) {
    block.classList.remove("hidden")
  } 
  else {
    block.classList.add("hidden");
  }
});
on('body', 'click', '#create_organization', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_director").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_phone").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название организации");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание организации");
    return
  }
  else if (!form.querySelector("#id_director").value) {
    form.querySelector("#id_director").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите директора организации");
    return
  }
  else if (!form.querySelector("#id_phone").value) {
    form.querySelector("#id_phone").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите телефон организации");
    return
  }

  form.querySelector("#create_organization").setAttribute("disabled", "true");
  form.querySelector("#create_organization").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_organization/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_place', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_title").value) {
      form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название кладбища");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну кладбища");
    return
  }

  form.querySelector("#create_place").setAttribute("disabled", "true");
  form.querySelector("#create_place").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_place/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '#create_brave', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_country").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_title").value) {
      form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название братской могилы");
      return
  }
  else if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну кладбища");
    return
  }

  form.querySelector("#create_brave").setAttribute("disabled", "true");
  form.querySelector("#create_brave").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_brave/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '#create_deceased', function() { 
  let form = this.parentElement.parentElement.parentElement;
  form.querySelector("#id_first_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_last_name").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_first_name").value) {
      form.querySelector("#id_first_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите имя усопшего");
      return
  }
  else if (!form.querySelector("#id_last_name").value) {
    form.querySelector("#id_last_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите фамилию усопшего");
    return
  }
  else if (!form.querySelector("#id_place").value) {
    form.querySelector("#id_place").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите место захоронения усопшего");
    return
  }
  else if (!form.querySelector("#id_cord").value) {
    form.querySelector("#id_cord").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите координаты усопшего");
    return
  }

  form.querySelector("#create_deceased").setAttribute("disabled", "true");
  form.querySelector("#create_deceased").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_deceased/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

on('body', 'click', '#edit_organization', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_director").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_phone").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_name").value) {
      form.querySelector("#id_name").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите название организации");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание организации");
    return
  }
  else if (!form.querySelector("#id_director").value) {
    form.querySelector("#id_director").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите директора организации");
    return
  }
  else if (!form.querySelector("#id_phone").value) {
    form.querySelector("#id_phone").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите телефон организации");
    return
  }

  form.querySelector("#edit_organization").setAttribute("disabled", "true");
  form.querySelector("#edit_organization").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_organization/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_loc', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#country_id").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_address2").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#country_id").value) {
      form.querySelector("#country_id").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите страну организации");
      return
  }
  else if (!form.querySelector("#id_address2").value) {
    form.querySelector("#id_address2").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите улицу и дом организации");
    return
  }

  form.querySelector("#create_loc").setAttribute("disabled", "true");
  form.querySelector("#create_loc").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_loc/" + this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#edit_loc', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_country").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_address2").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_country").value) {
    form.querySelector("#id_country").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите страну организации");
    return
  }
  else if (!form.querySelector("#id_address2").value) {
    form.querySelector("#id_address2").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите улицу и дом организации");
    return
  }

  form.querySelector("#edit_loc").setAttribute("disabled", "true");
  form.querySelector("#edit_loc").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_loc/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});

on('body', 'click', '#create_service', function() {
  let form = this.parentElement.parentElement.parentElement.parentElement.parentElement;

  form.querySelector("#id_price").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_price").value) {
      form.querySelector("#id_price").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите цену услуги");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание услуги");
    return
  }
  else if (!form.querySelector("#id_title").value) {
    form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите название услуги");
    return
  }

  form.querySelector("#create_service").setAttribute("disabled", "true");
  form.querySelector("#create_service").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/create_service/" + this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }}; 
    link.send(form_data);
});

on('body', 'click', '#edit_service', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement.parentElement.parentElement;
  
  form.querySelector("#id_price").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_description").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_title").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_price").value) {
      form.querySelector("#id_price").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите цену услуги");
      return
  }
  else if (!form.querySelector("#id_description").value) {
    form.querySelector("#id_description").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите описание услуги");
    return
  }
  else if (!form.querySelector("#id_title").value) {
    form.querySelector("#id_title").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите название услуги");
    return
  }

  form.querySelector("#edit_service").setAttribute("disabled", "true");
  form.querySelector("#edit_service").innerHTML = "Идет сохранение";
  
  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_service/" + _this.getAttribute("data-pk") + "/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      window.location.href = '/organization/' + link.responseText + "/";
    }};
    link.send(form_data);
});


function delete_item_2(url, id) {
  form_data = new FormData();
  form_data.append("id", id);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true )
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    toast_success("Удалено!");
  }};
  link.send(form_data);
};

on('body', 'click', '.delete_service', function() {
  delete_item("/delete_service/", this.getAttribute("data-pk"));
  this.parentElement.parentElement.parentElement.remove();
});
on('body', 'click', '.delete_loc', function() {
  delete_item("/delete_loc/", this.getAttribute("data-pk"));
  this.parentElement.parentElement.remove();
});

on('body', 'click', '#images_container', function() {
  this.previousElementSibling.click();
});

on('body', 'change', '#pro-images', function() {
  len = this.files.length;
  if (len > 10) {
    alert("Максимальное количество фотографий - 10")
  }

  a = len % 10;
  b = len % 100;

  if (a == 1 && b != 11) {
    word = "Выбранa " + len + " фотография";
  }
  else if (a >= 2 && a <= 4 && (b < 10 || b >= 20)) {
    word = "Выбраны " + len + " фотографии";
  }
  else {
    word = "Выбраны " + len + " фотографий";
  }

  document.body.querySelector(".photos_upload_response").innerHTML = word;
}); 

on('body', 'click', '.remove_file', function() {
  delete_item("/delete_file/", this.getAttribute("data-pk"));
  this.parentElement.remove();
}); 


on('body', 'click', '#edit_profile', function() {
  _this = this;
  form = _this.parentElement.parentElement.parentElement;
  form.querySelector("#id_username").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_first_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_last_name").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_phone").style.setProperty('border', 'inherit', 'important');
  form.querySelector("#id_email").style.setProperty('border', 'inherit', 'important');

  if (!form.querySelector("#id_username").value) {
      form.querySelector("#id_username").style.setProperty('border', '1px #FF0000 solid', 'important');
      toast_error("Укажите логин");
      return
  }
  else if (!form.querySelector("#id_first_name").value) {
    form.querySelector("#id_first_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите имя усопшего");
    return
  }
  else if (!form.querySelector("#id_last_name").value) {
    form.querySelector("#id_last_name").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите фамилию усопшего");
    return
  }
  else if (!form.querySelector("#id_phone").value) {
    form.querySelector("#id_phone").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите телефон");
    return
  }
  else if (!form.querySelector("#id_email").value) {
    form.querySelector("#id_email").style.setProperty('border', '1px #FF0000 solid', 'important');
    toast_error("Укажите почту");
    return
  }
  
  form.querySelector("#edit_profile").setAttribute("disabled", "true");
  form.querySelector("#edit_profile").innerHTML = "Идет сохранение";

  form_data = new FormData(form);
  
    link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
    link.open( 'POST', "/edit_profile/", true );
    link.onreadystatechange = function () {
    if ( link.readyState == 4 && link.status == 200 ) {
      location.reload()
    }};
    link.send(form_data);
});

//////////////////////////////////
//////////////////////////////////
//////////////////////////////////
on('body', 'click', '.body_overlay', function() {
  close_fullscreen();
});
on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen();
});
function get_document_opacity_0() {
  document.body.style.overflowY = "hidden";
  //document.body.style.marginRight = "20px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflowY = "scroll";
  //document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  if (!container.innerHTML) {
    get_document_opacity_1();
    return
  };
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");

  _window.remove();

  if (!container.innerHTML) {
    get_document_opacity_1();
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.classList.remove("hide");
  };
};

var delayedExec = function(after, fn) {
  var timer;
  return function() {
      timer && clearTimeout(timer);
      timer = setTimeout(fn, after);
  };
};

function scrolled(_block) {
  offset = 0;
  window.onscroll = function() {
    try {
        box = _block.querySelector('.next_page_list');
        if (box && box.classList.contains("next_page_list")) {
            inViewport = elementInViewport(box);
            if (inViewport) {
                box.classList.remove("next_page_list");
                paginate(box);
            }
        };
    } catch {return}
  }
};
function paginate(block) {
      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
      link_3.open('GET', location.protocol + "//" + location.host + block.getAttribute("data-link") + "&ajax=2", true);
      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

      link_3.onreadystatechange = function() {
          if (this.readyState == 4 && this.status == 200) {
              var elem = document.createElement('span');
              elem.innerHTML = link_3.responseText;
              block.parentElement.insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML)
              block.remove()
          }
      }
      link_3.send();
};

function elementInViewport(el){var bounds = el.getBoundingClientRect();return ((bounds.top + bounds.height > 0) && (window.innerHeight - bounds.top > 0));}


function create_fullscreen(url, type_class, deseaced_map, place_map, org_map, cord) {
  container = document.body.querySelector("#fullscreens_container");

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
    if (this.readyState == 4 && this.status == 200) {
      if (container.innerHTML) {
        prev_window = container.querySelector(".card_fullscreen");
        prev_window.classList.add("hide");
      };

      $parent_div = document.createElement("div");
      $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
      $parent_div.style.zIndex = 100 + count_items;
      $parent_div.style.opacity = "0";

      $hide_span = document.createElement("span");
      $hide_span.classList.add("this_fullscreen_hide");
      $loader = document.createElement("div");

      $loader.setAttribute("id", "fullscreen_loader");
      $hide_span.innerHTML = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>';
      $parent_div.append($hide_span);
      $parent_div.append($loader);
      container.prepend($parent_div);

        elem = link.responseText;

        $loader.innerHTML = elem;
        height = $loader.scrollHeight*1 + 30; 
        if (height < 500) {
          if (!cord) {
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
          }
          else {
            $parent_div.style.height = "800px";
            $parent_div.style.top = "10%";
          }
          $loader.style.overflowY = "unset";
        } else {
          $parent_div.style.height = "100%";
          $parent_div.style.top = "15px";
          $loader.style.overflowY = "auto";
        };
        $parent_div.style.opacity = "1";
        if ($loader.querySelector(".data_display")) {
          $loader.style.overflowY = "unset";
        }

        get_document_opacity_0();

        offset = 0;

        $loader.onscroll = function() {
          if ($loader.scrollHeight > offset) {
            offset = $loader.scrollHeight;
            $window_height = parseFloat(offset * 0.000264).toFixed(2);
          }
          if ($loader.querySelector(".next_page_list")) {
            box = $loader.querySelector('.next_page_list');
            if (box && box.classList.contains("next_page_list")) {
                inViewport = elementInViewport(box);
                if (inViewport) {
                    box.remove();
                    var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                    link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link") + "&ajax=2", true);
                    link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                    link_3.onreadystatechange = function() {
                        if (this.readyState == 4 && this.status == 200) {
                            var elem = document.createElement('span');
                            elem.innerHTML = link_3.responseText;
                            $loader.querySelector(".is_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_paginate").innerHTML);
                          }
                    }
                    link_3.send();
                }
            };
          }
        };
        if (deseaced_map) {
          load_deceadeds_map(cord)
        }
        else if (place_map) {
          load_places_map(cord)
        };
    }
};
link.send();
};
 

on('body', 'click', '.show_deceased_map', function() {
  create_fullscreen ("/deceased/" + this.getAttribute("data-pk") + "/map/" , "photo_fullsvreen", true, false, false, this.getAttribute("data-cord"))
});
on('body', 'click', '.show_place_map', function() {
  create_fullscreen ("/place/" + this.getAttribute("data-pk") + "/map/" , "photo_fullsvreen", false, true, false, this.getAttribute("data-cord"))
}); 
on('body', 'click', '.show_org_map', function() {
  create_fullscreen ("/organization/" + this.getAttribute("data-pk") + "/map/" , "photo_fullsvreen", false, false, true, this.getAttribute("data-cord"))
});

on('body', 'click', 'button', function() {
  let block = this.nextElementSibling;
  if (block && block.classList.contains("dropdown-menu")) {
    if (block.classList.contains("show")) {
      block.classList.remove("show");
    }
    else {
      block.classList.add("show");
    }
  }
  else {
    list = document.body.querySelectorAll(".dropdown-menu");
    for (var i = 0, len = list.length; i < len; i++) {
      list[i].classList.remove("show");
    }
  }
});