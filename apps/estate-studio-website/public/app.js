const data = JSON.parse(document.querySelector('#page-data').textContent);
let opener;
for (const button of document.querySelectorAll('[data-open]')) {
 button.addEventListener('click', () => {
  opener = button;
  document.getElementById(button.dataset.open).showModal();
  document.body.classList.add('dialog-open');
 });
}
for (const dialog of document.querySelectorAll('dialog')) {
 dialog.querySelector('[data-close]').addEventListener('click', () => dialog.close());
 dialog.addEventListener('click', e => { if (e.target === dialog) {
  const r = dialog.getBoundingClientRect();
  if (e.clientX < r.left || e.clientX > r.right || e.clientY < r.top || e.clientY > r.bottom) dialog.close();
 }});
 dialog.addEventListener('close', () => { document.body.classList.remove('dialog-open'); opener?.focus(); });
}
const sources = ['/assets/living.webp','/assets/kitchen.webp','/assets/bedroom1.webp'];
for (const button of document.querySelectorAll('[data-room]')) {
 button.addEventListener('click', () => {
  const index = Number(button.dataset.room);
  const img = document.getElementById('room-image');
  img.src = sources[index]; img.alt = data.roomAlts[index];
  document.getElementById('room-label').textContent = data.rooms[index];
  for (const sibling of document.querySelectorAll('[data-room]')) sibling.setAttribute('aria-pressed', String(sibling === button));
 });
}
document.getElementById('demo-form').addEventListener('submit', e => {
 e.preventDefault();
 const values = new FormData(e.currentTarget);
 const body = ['name','email','company','project'].map((key,i) => `${data.fields[i]}: ${String(values.get(key) || '').trim()}`).join('\r\n');
 // No network request or storage. The visitor must send the draft in their mail application.
 window.location.href = `mailto:hello@estatestudio.io?subject=${encodeURIComponent(data.subject)}&body=${encodeURIComponent(body)}`;
 document.getElementById('form-feedback').hidden = false;
});
