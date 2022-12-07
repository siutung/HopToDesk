lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Статус"),
        ("Your Desktop", "Ваш робочий стіл"),
        ("desk_tip", "Ваш робочий стіл доступний з цим ідентифікатором і паролем"),
        ("Password", "Пароль"),
        ("Ready", "Готово"),
        ("Established", "Встановлено"),
        ("connecting_status", "Підключення до мережі HopToDesk..."),
		("connecting_status_short", "Підключення..."),				
        ("Enable Service", "Включити службу"),
        ("Start Service", "Запустити службу"),
        ("Service is running", "Служба працює"),
        ("Service is not running", "Служба не запущена"),
        ("not_ready_status", "Не готово. Будь ласка, перевірте підключення"),
		("not_ready_status_short", "Не готово"),						
        ("Control Remote Desktop", "Управління віддаленим робочим столом"),
        ("Transfer File", "Передати файл"),
        ("Connect", "Підключитися"),
        ("Recent Sessions", "Останні сеанси"),
        ("Address Book", "Адресна книга"),
        ("Confirmation", "Підтвердження"),
        ("TCP Tunneling", "TCP-тунелювання"),
        ("Remove", "Видалити"),
        ("Refresh random password", "Оновити випадковий пароль"),
        ("Set your own password", "Встановити свій пароль"),
        ("Enable Keyboard/Mouse", "Увімкнути клавіатуру/мишу"),
        ("Enable Clipboard", "Увімкнути буфер обміну"),
        ("Enable File Transfer", "Увімкнути передачу файлів"),
        ("Enable TCP Tunneling", "Увімкнути тунелювання TCP"),
        ("IP Whitelisting", "Список дозволених IP-адрес"),
        ("ID/Relay Server", "ID/Сервер ретрансляції"),
        ("Import Server Conf", "Імпортувати конфігурацію сервера"),
        ("Import server configuration successfully", "Конфігурацію сервера успішно імпортовано"),
        ("Invalid server configuration", "Недійсна конфігурація сервера"),
        ("Clipboard is empty", "Буфер обміну порожній"),
        ("Stop service", "Зупинити службу"),
        ("Change ID", "Змінити ID"),
        ("Website", "Веб-сайт"),
        ("About", "Про HopToDesk"),
        ("Mute", "Вимкнути звук"),
        ("Audio Input", "Аудіовхід"),
        ("Enhancements", "Покращення"),
        ("Hardware Codec", "Апаратний кодек"),
        ("Adaptive Bitrate", "Адаптивна швидкість потоку"),
        ("ID Server", "ID-сервер"),
        ("Relay Server", "Сервер ретрансляції"),
        ("API Server", "API-сервер"),
        ("invalid_http", "Повинен починатися з http:// або https://"),
        ("Invalid IP", "Невірна IP-адреса"),
        ("id_change_tip", "Допускаються тільки символи a-z, A-Z, 0-9 і _ (підкреслення). Перша буква повинна бути a-z, A-Z. Довжина від 6 до 16"),
        ("Invalid format", "Невірний формат"),
        ("server_not_support", "Поки не підтримується сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Занадто часто"),
        ("Cancel", "Скасувати"),
        ("Skip", "Пропустити"),
        ("Close", "Закрити"),
        ("Retry", "Спробувати знову"),
        ("OK", "ОК"),
        ("Password Required", "Потрібен пароль"),
        ("Please enter your password", "Будь ласка, введіть ваш пароль"),
        ("Remember password", "Запам'ятати пароль"),
        ("Wrong Password", "Невірний пароль"),
        ("Do you want to enter again?", "Ви хочете знову увійти?"),
        ("Connection Error", "Помилка підключення"),
        ("Error", "Помилка"),
        ("Connection lost", "Скинуто піром"),
        ("Connecting...", "Підключення..."),
        ("Connection in progress. Please wait.", "Виконується підключення. Будь ласка, зачекайте."),
        ("Please try 1 minute later", "Спробуйте через 1 хвилину"),
        ("Login Error", "Помилка входу"),
        ("Successful", "Операція успішна"),
        ("Connected, waiting for image...", "Підключено, очікування зображення..."),
        ("Name", "Ім'я"),
        ("Type", "Тип"),
        ("Modified", "Змінено"),
        ("Size", "Розмір"),
        ("Show Hidden Files", "Показати приховані файли"),
        ("Receive", "Отримати"),
        ("Send", "Відправити"),
        ("Refresh File", "Оновити файл"),
        ("Local", "Локальний"),
        ("Remote", "Віддалений"),
        ("Remote Computer", "Віддалений комп'ютер"),
        ("Local Computer", "Локальний комп'ютер"),
        ("Confirm Delete", "Підтвердити видалення"),
        ("Delete", "Видалити"),
        ("Properties", "Властивості"),
        ("Multi Select", "Багатоелементний вибір"),
        ("Empty Directory", "Порожня папка"),
        ("Not an empty directory", "Папка не порожня"),
        ("Are you sure you want to delete this file?", "Ви впевнені, що хочете видалити цей файл?"),
        ("Are you sure you want to delete this empty directory?", "Ви впевнені, що хочете видалити порожню папку?"),
        ("Are you sure you want to delete the file of this directory?", "Ви впевнені, що хочете видалити файл із цієї папки?"),
        ("Do this for all conflicts", "Це стосується всіх конфліктів"),
        ("This is irreversible!", "Це незворотньо!"),
        ("Deleting", "Видалення"),
        ("files", "файли"),
        ("Waiting", "Очікування"),
        ("Finished", "Завершено"),
        ("Speed", "Швидкість"),
        ("Custom Image Quality", "Користувацька якість зображення"),
        ("Privacy mode", "Режим конфіденційності"),
        ("Block user input", "Блокувати користувацьке введення"),
        ("Unblock user input", "Розблокувати введення користувача"),
        ("Adjust Window", "Налаштувати вікно"),
        ("Original", "Оригінал"),
        ("Shrink", "Зменшити"),
        ("Stretch", "Розтягнути"),
        ("Scrollbar", "Смуга прокрутки"),
        ("ScrollAuto", "Прокрутка Авто"),
        ("Good image quality", "Хороша якість зображення"),
        ("Balanced", "Збалансований"),
        ("Optimize reaction time", "Оптимізувати час реакції"),
        ("Custom", "Користувацький"),
        ("Show remote cursor", "Показати віддалений курсор"),
        ("Show quality monitor", "Показати якість"),
        ("Disable clipboard", "Відключити буфер обміну"),
        ("Lock after session end", "Вихід з облікового запису після завершення сеансу"),
        ("Insert", "Вставити"),
        ("Insert Lock", "Встановити замок"),
        ("Refresh", "Оновити"),
        ("ID does not exist", "ID не існує"),
        ("Failed to connect to rendezvous server", "Не вдалося підключитися до проміжного сервера"),
        ("Please try later", "Будь ласка, спробуйте пізніше"),
        ("Remote desktop is offline", "Віддалений робочий стіл не в мережі"),
        ("Key mismatch", "Невідповідність ключів"),
        ("Timeout", "Тайм-аут"),
        ("Failed to connect to relay server", "Не вдалося підключитися до сервера ретрансляції"),
        ("Failed to connect via rendezvous server", "Не вдалося підключитися через проміжний сервер"),
        ("Failed to connect via relay server", "Не вдалося підключитися через сервер ретрансляції"),
        ("Failed to make direct connection to remote desktop", "Не вдалося встановити пряме підключення до віддаленого робочого столу"),
        ("Set Password", "Встановити пароль"),
        ("OS Password", "Пароль ОС"),
        ("install_tip", "Рекомендується повне встановлення."),
        ("Click to upgrade", "Натисніть, щоб перевірити наявність оновлень"),
        ("Click to download", "Натисніть, щоб завантажити"),
        ("Click to update", "Натисніть, щоб оновити"),
        ("Configure", "Налаштувати"),
        ("config_acc", "Щоб віддалено керувати своїм робочим столом, ви повинні надати HopToDesk права \"доступу\""),
        ("config_screen", "Для віддаленого доступу до робочого столу ви повинні надати HopToDesk права \"знімок екрану\""),
        ("Installing ...", "Встановлюється..."),
        ("Install", "Встановити"),
        ("Installation", "Установка"),
        ("Installation Path", "Шлях встановлення"),
        ("Create start menu shortcuts", "Створити ярлики меню \"Пуск\""),
        ("Create desktop icon", "Створити значок на робочому столі"),
        ("agreement_tip", "Починаючи установку, ви приймаєте умови ліцензійної угоди"),
        ("Accept and Install", "Прийняти і встановити"),
        ("End-user license agreement", "Ліцензійна угода з кінцевим користувачем"),
        ("Generating ...", "Генерація..."),
        ("Your installation is lower version.", "Ваша установка більш ранньої версії"),
        ("not_close_tcp_tip", "Не закривати це вікно під час використання тунелю"),
        ("Listening ...", "Очікуємо ..."),
        ("Remote Host", "Віддалена машина"),
        ("Remote Port", "Віддалений порт"),
        ("Action", "Дія"),
        ("Add", "Додати"),
        ("Local Port", "Локальний порт"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Для більш швидкого підключення налаштуйте свій власний сервер підключення"),
        ("Too short, at least 6 characters.", "Занадто коротко, мінімум 6 символів"),
        ("The confirmation is not identical.", "Підтвердження не збігається"),
        ("Permissions", "Дозволи"),
        ("Accept", "Прийняти"),
        ("Dismiss", "Відхилити"),
        ("Disconnect", "Відключити"),
        ("Allow using keyboard and mouse", "Дозволити використання клавіатури і миші"),
        ("Allow using clipboard", "Дозволити використання буфера обміну"),
        ("Allow hearing sound", "Дозволити передачу звуку"),
        ("Allow file copy and paste", "Дозволити копіювання і вставку файлів"),
        ("Connected", "Підключено"),
        ("Direct and encrypted connection", "Пряме і зашифроване з'єднання"),
        ("Relayed and encrypted connection", "Ретрансльоване і зашифроване з'єднання"),
        ("Direct and unencrypted connection", "Пряме і незашифроване з'єднання"),
        ("Relayed and unencrypted connection", "Ретрансльоване і незашифроване з'єднання"),
        ("Enter Remote ID", "Введіть віддалений ID"),
        ("Enter your password", "Введіть пароль"),
        ("Logging in...", "Вхід..."),
        ("Enable RDP session sharing", "Включити загальний доступ до сеансу RDP"),
        ("Auto Login", "Автоматичний вхід (дійсний, тільки якщо ви встановили \"Завершення користувацького сеансу після завершення віддаленого підключення\")"),
        ("Enable Direct IP Access", "Увімкнути прямий IP-доступ"),
        ("Rename", "Перейменувати"),
        ("Space", "Місце"),
        ("Create Desktop Shortcut", "Створити ярлик на робочому столі"),
        ("Change Path", "Змінити шлях"),
        ("Create Folder", "Створити папку"),
        ("Please enter the folder name", "Будь ласка, введіть ім'я папки"),
        ("Fix it", "Виправити"),
        ("Warning", "Попередження"),
        ("Login screen using Wayland is not supported", "Вхід у систему з використанням Wayland не підтримується"),
        ("Reboot required", "Потрібне перезавантаження"),
        ("Unsupported display server ", ""),
        ("x11 expected", "Очікується X11"),
        ("Port", "Порт"),
        ("Settings", "Налаштування"),
        ("Username", "Ім'я користувача"),
        ("Invalid port", "Неправильний порт"),
        ("The remote partner has closed the session.", "Закрито вузлом вручну"),
        ("Enable remote configuration modification", "Дозволити віддалену зміну конфігурації"),
        ("Run without install", "Запустити без установки"),
        ("Always connected via relay", "Завжди підключений через ретрансляційний сервер"),
        ("Always connect via relay", "Завжди підключатися через ретрансляційний сервер"),
        ("whitelist_tip", "Тільки IP-адреси з білого списку можуть отримати доступ до мене"),
        ("Login", "Увійти"),
        ("Logout", "Вийти"),
        ("Tags", "Ключові слова"),
        ("Search ID", "Пошук за ID"),
        ("Current Wayland display server is not supported", "Поточний сервер відображення Wayland не підтримується"),
        ("whitelist_sep", "Окремо комою, крапкою з комою, пропуском або новим рядком"),
        ("Add ID", "Додати ID"),
        ("Add Tag", "Додати ключове слово"),
        ("Unselect all tags", "Скасувати вибір усіх тегів"),
        ("Network error", "Помилка мережі"),
        ("Username missed", "Ім'я користувача відсутнє"),
        ("Password missed", "Забули пароль"),
        ("Wrong credentials", "Невірні облікові дані"),
        ("Edit Tag", "Редагувати тег"),
        ("Forget Password", "Не зберігати пароль"),
        ("Favorites", "Вибране"),
        ("Add to Favorites", "Додати в обране"),
        ("Remove from Favorites", "Видалити з обраного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Недопустиме ім'я папки"),
        ("SOCKS5 Proxy", "Проксі-сервер SOCKS5"),
        ("Hostname", "Ім'я ПК"),
        ("Discovered", "Знайдено"),
        ("install_daemon_tip", "Для запуску під час завантаження необхідно встановити системну службу"),
        ("Remote ID", "Віддалений ідентифікатор"),
        ("Paste", "Вставити"),
        ("Paste here?", "Вставити сюди?"),
        ("Are you sure to close the connection?", "Ви впевнені, що хочете завершити підключення?"),
        ("Download new version", "Завантажити нову версію"),
        ("Touch mode", "Сенсорний режим"),
        ("Mouse mode", "Режим миші"),
        ("One-Finger Tap", "Дотик одним пальцем"),
        ("Left Mouse", "Ліва кнопка миші"),
        ("One-Long Tap", "Одне довге натискання пальцем"),
        ("Two-Finger Tap", "Дотик двома пальцями"),
        ("Right Mouse", "Права миша"),
        ("One-Finger Move", "Рух одним пальцем"),
        ("Double Tap & Move", "Подвійне натискання і переміщення"),
        ("Mouse Drag", "Перетягування мишею"),
        ("Three-Finger vertically", "Трьома пальцями по вертикалі"),
        ("Mouse Wheel", "Коліщатко миші"),
        ("Two-Finger Move", "Рух двома пальцями"),
        ("Canvas Move", "Переміщення полотна"),
        ("Pinch to Zoom", "Стисніть, щоб збільшити"),
        ("Canvas Zoom", "Масштаб полотна"),
        ("Reset canvas", "Скинути полотно"),
        ("No permission of file transfer", "Немає дозволу на передачу файлів"),
        ("Note", "Примітка"),
        ("Connection", "З'єднання"),
        ("Share Screen", "Поділитися екраном"),
        ("CLOSE", "ЗАКРИТИ"),
        ("OPEN", "ВІДКРИТИ"),
        ("Chat", "Чат"),
        ("Total", "Всього"),
        ("items", "елементи"),
        ("Selected", "Обрано"),
        ("Screen Capture", "Захоплення екрана"),
        ("Input Control", "Вхідний контроль"),
        ("Audio Capture", "Захоплення аудіо"),
        ("File Connection", "Файлове з'єднання"),
        ("Screen Connection", "Підключення екрана"),
        ("Do you accept?", "Ви згодні?"),
        ("Open System Setting", "Відкрити налаштування системи"),
        ("How to get Android input permission?", "Як отримати дозвіл на введення Android?"),
        ("android_input_permission_tip1", "Щоб віддалений пристрій міг керувати вашим Android-пристроєм за допомогою миші або торкання, вам необхідно дозволити HopToDesk використовувати службу \"Спеціальні можливості\"."),
        ("android_input_permission_tip2", "Перейдіть на наступну сторінку системних налаштувань, знайдіть і увійдіть у [Встановлені служби], увімкніть службу [HopToDesk Input]."),
        ("android_new_connection_tip", "Отримано новий запит на управління вашим поточним пристроєм."),
        ("android_service_will_start_tip", "Увімкнення захоплення екрана автоматично запускає службу, дозволяючи іншим пристроям запитувати з'єднання з цього пристрою."),
        ("android_stop_service_tip", "Закриття служби автоматично закриє всі встановлені з'єднання."),
        ("android_version_audio_tip", "Поточна версія Android не підтримує захоплення звуку, оновіть її до Android 10 або вище."),
        ("android_start_service_tip", "Натисніть [Запуск проміжного сервера] або ВІДКРИТИ роздільну здатність [Захоплення екрана], щоб запустити службу демонстрації екрана."),
        ("Account", "Акаунт"),
        ("Overwrite", "Перезаписати"),
        ("This file exists, skip or overwrite this file?", "Цей файл існує, пропустити або перезаписати файл?"),
        ("Quit", "Вийти"),
        ("doc_mac_permission", ""),
        ("Help", "Допомога"),
        ("Failed", "Не вдалося"),
        ("Succeeded", "Успішно"),
        ("Someone turns on privacy mode, exit", "Хтось вмикає режим конфіденційності, вихід"),
        ("Unsupported", "Не підтримується"),
        ("Peer denied", "Відхилено віддаленим комп'ютером"),
        ("Please install plugins", "Будь ласка, встановіть плагіни"),
        ("Peer exit", "Відключено віддаленим комп'ютером"),
        ("Failed to turn off", "Не вдалося вимкнути"),
        ("Turned off", "Вимкнений"),
        ("In privacy mode", "У режимі конфіденційності"),
        ("Out privacy mode", "Вихід із режиму конфіденційності"),
        ("Language", "Мова"),
        ("Keep HopToDesk background service", "Зберегти фонову службу HopToDesk"),
        ("Ignore Battery Optimizations", "Ігнорувати оптимізацію батареї"),
        ("android_open_battery_optimizations_tip", "Перейдіть на наступну сторінку налаштувань"),
        ("Connection not allowed", "Підключення не дозволено"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", "Використовувати постійний пароль"),
        ("Use both passwords", "Використовувати обидва паролі"),
        ("Set permanent password", "Встановити постійний пароль"),
        ("Enable Remote Restart", "Увімкнути віддалений перезапуск"),
        ("Allow remote restart", "Дозволити віддалений перезапуск"),
        ("Restart Remote Device", "Перезапустити віддалений пристрій"),
        ("Are you sure you want to restart", "Ви впевнені, що хочете виконати перезапуск?"),
        ("Restarting Remote Device", "Перезавантаження віддаленого пристрою"),
        ("remote_restarting_tip", "Віддалений пристрій перезапускається. Будь ласка, закрийте це повідомлення і через деякий час перепідключіться, використовуючи постійний пароль."),
        ("Copied", ""),
        ("Exit Fullscreen", "Вийти з повноекранного режиму"),
        ("Fullscreen", "Повноекранний"),
        ("Mobile Actions", "Мобільні дії"),
        ("Select Monitor", "Виберіть монітор"),
        ("Control Actions", "Дії з управління"),
        ("Display Settings", "Налаштування відображення"),
        ("Ratio", "Співвідношення"),
        ("Image Quality", "Якість зображення"),
        ("Scroll Style", "Стиль прокрутки"),
        ("Show Menubar", "Показати рядок меню"),
        ("Hide Menubar", "приховати рядок меню"),
        ("Direct Connection", "Прямий зв'язок"),
        ("Relay Connection", "Релейне з'єднання"),
        ("Secure Connection", "Безпечне з'єднання"),
        ("Insecure Connection", "Небезпечне з'єднання"),
        ("Scale original", "Оригінал масштабу"),
        ("Scale adaptive", "Масштаб адаптивний"),
        ("General", "Загальні"),
        ("Security", "Безпека"),
        ("Account", "Акаунт"),
        ("Theme", "Тема"),
        ("Dark Theme", "Темна тема"),
        ("Dark", "Темна"),
        ("Light", "Світла"),
        ("Follow System", "Використовувати системну"),
        ("Enable hardware codec", "Увімкнути апаратний кодек"),
        ("Unlock Security Settings", "Розблокувати налаштування безпеки"),
        ("Enable Audio", "Вімкнути аудіо"),
        ("Unlock Network Settings", "Розблокувати мережеві налаштування"),
        ("Server", "Сервер"),
        ("Direct IP Access", "Прямий IP доступ"),
        ("Proxy", "Проксі"),
        ("Port", "Порт"),
        ("Apply", "Застосувати"),
        ("Disconnect all devices?", "Відключити всі прилади?"),
        ("Clear", "Очистити"),
        ("Audio Input Device", "Пристрій введення звуку"),
        ("Deny remote access", "Заборонити віддалений доступ"),
        ("Use IP Whitelisting", "Використовувати білий список IP"),
        ("Network", "Мережа"),
        ("Enable RDP", " Увімкнути RDP"),
        ("Pin menubar", "Закріпити рядок меню"),
        ("Unpin menubar", "Відкріпити рядок меню"),
        ("Recording", "Запис"),
        ("Directory", "Директорія"),
        ("Automatically record incoming sessions", "Автоматично записувати вхідні сеанси"),
        ("Change", "Змінити"),
        ("Start session recording", "Розпочати запис сесії"),
        ("Stop session recording", "Закінчити запис сесії"),
        ("Enable Recording Session", "Увімкнути запис сесії"),
        ("Allow recording session", "Дозволити запис сеансу"),
        ("Enable LAN Discovery", "Увімкнути пошук локальної мережі"),
        ("Deny LAN Discovery", "Заборонити виявлення локальної мережі"),
        ("Write a message", "Написати повідомлення"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Custom", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland потребує Ubuntu 21.04 або новішої версії."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Для Wayland потрібна новіша версія дистрибутива Linux. Будь ласка, спробуйте робочий стіл X11 або змініть свою ОС."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Будь ласка, виберіть екран, до якого потрібно надати доступ (працюйте на стороні однорангового пристрою)."),
        ("Show HopToDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
		("Enable Wake On LAN", "Увімкніть Wake On LAN"),
		("Enable 2FA", "Увімкніть 2FA"),
		("2FA QR Code", "QR-код 2FA"),
        ("Scan this QR code with a camera on a secondary device such as a phone to set it up as your 2FA authenticator.", "Відскануйте цей QR-код за допомогою камери на додатковому пристрої, наприклад телефоні, щоб налаштувати його як автентифікатор 2FA."),
        ("You will need to confirm the 2FA on the secondary device with you when trying to connect to this desktop.", "Вам потрібно буде підтвердити 2FA на додатковому пристрої, коли ви намагаєтеся підключитися до цього робочого столу."),		
    ].iter().cloned().collect();
}
