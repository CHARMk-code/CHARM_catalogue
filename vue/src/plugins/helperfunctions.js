function cookiesAccepted() {
  return (
    RegExp("cookieconsent_status" + "=[^;]+").exec(document.cookie) != null
  );
}

export default { cookiesAccepted };
