-app-name = Vizia

system-theme = النظام
dark-theme = داكن
light-theme = فاتح
blue = أزرق
emerald = زمردي
crimson = قرمزي
amber = كهرماني
violet = بنفسجي
en = الإنجليزية
fr = الفرنسية
ar = العربية
button = زر
secondary-button = زر ثانوي
outline-button = زر بإطار
text-button = زر نصي
button-with-icon = زر مع أيقونة
checkbox = صندوق اختيار
one = واحد
two = اثنان
three = ثلاثة
toggle-disabled = التبديل معطل
default = افتراضي

## Example message with attributes
form-submission = إرسال النموذج
    .label = إرسال النموذج
    .help-text = جميع الحقول مطلوبة
    .success-message = تم إرسال النموذج بنجاح

## Number formatting examples
item-count = لديك { $count } عناصر
discount-percent = الخصم: { $discount }%

## Date formatting examples
joined-date = عضو منذ { DATETIME($date, year: "numeric", month: "long", day: "numeric") }
last-login = آخر دخول: { DATETIME($date, month: "short", day: "numeric", hour: "2-digit", minute: "2-digit") }

## Calendar
calendar-previous-month = الشهر السابق
calendar-next-month = الشهر التالي
calendar-week-start = sunday
calendar-month-year-heading = { DATETIME($date, year: "numeric", month: "long") }
calendar-day-cell-name = { DATETIME($date, weekday: "long", year: "numeric", month: "long", day: "numeric") }
calendar-keyboard-help = استخدم مفاتيح الأسهم للتنقل بين الأيام، ومفتاحي Home وEnd لبداية ونهاية الأسبوع، وPage Up وPage Down للشهر، وShift مع Page Up وPage Down للسنة، وEnter أو Space للاختيار.

Jan = يناير
Feb = فبراير
Mar = مارس
Apr = أبريل
May = مايو
Jun = يونيو
Jul = يوليو
Aug = أغسطس
Sept = سبتمبر
Oct = أكتوبر
Nov = نوفمبر
Dec = ديسمبر

Monday = الاثنين
Tuesday = الثلاثاء
Wednesday = الأربعاء
Thursday = الخميس
Friday = الجمعة
Saturday = السبت
Sunday = الأحد

Monday-short = اثن
Tuesday-short = ثلا
Wednesday-short = أرب
Thursday-short = خمي
Friday-short = جمع
Saturday-short = سبت
Sunday-short = أحد

## Accordion
accordion-title-1 = نظرة عامة على المشروع
accordion-content-1 = Vizia عبارة عن إطار عمل واجهة مستخدم تصريحي لتطبيقات سطح المكتب.
accordion-title-2 = التثبيت
accordion-content-2 = أضف `vizia` إلى المتطلبات الخاصة بك وقم بتشغيل التطبيق.
accordion-title-3 = التصميم
accordion-content-3 = استخدم أوراق نمط من نوع CSS وأجهزة اختيار الفئات لتخصيص واجهة المستخدم الخاصة بك.
allow-multiple-open = السماح بفتح عدة أقسام
toggle-section = تبديل القسم الثاني

## View example window titles
view-title-textbox = حقل نص
view-title-scrollview = عرض قابل للتمرير
view-title-progressbar = شريط التقدم
view-title-label = تسمية
view-title-menubar = شريط القوائم
view-title-button-group = مجموعة الأزرار
view-title-vstack = مكدس عمودي
view-title-svg = SVG
view-title-collapsible = عنصر قابل للطي
view-title-resizable = قابل لتغيير الحجم
view-title-xypad = لوحة XY
view-title-radiobutton = زر اختيار
view-title-card = بطاقة
view-title-tabview = عرض التبويبات
view-title-virtual-table = جدول افتراضي لمجموعة بيانات كبيرة
view-title-tooltip = تلميح
view-title-knob = مقبض
view-title-toggle-button = زر تبديل
view-title-markdown = Markdown
view-title-menu = قائمة
view-title-list = قائمة
view-title-divider = فاصل
view-title-accordion = الأكورديون
view-title-hstack = مكدس أفقي
view-title-slider = منزلق
view-title-spinbox = مربع تدوير
view-title-dropdown = قائمة منسدلة
view-title-calendar = تقويم
view-title-avatar = الصورة الرمزية
view-title-table = جدول
view-title-chip = شارة
view-title-select = اختيار
view-title-virtual-list = قائمة افتراضية
view-title-switch = مفتاح تبديل
view-title-zstack = مكدس Z
view-title-combobox = مربع تحرير وسرد
view-title-rating = تقييم

## Textbox and scrolling
textbox-placeholder-type = اكتب شيئًا...
textbox-placeholder-search = بحث
scroll-vertical = تمرير عمودي
scroll-horizontal = تمرير أفقي
scroll-horizontal-vertical = تمرير أفقي وعمودي

## Label, checkbox, switch, and select examples
label-static-unicode = يمكن للتسمية عرض نص Unicode ثابت
label-wrap-enabled = سيتم التفاف النص إذا كان أطول من عرض التسمية.
label-wrap-disabled = إلا إذا تم تعطيل التفاف النص.
label-describing-trigger = التسمية التي تصف عنصر نموذج تعمل أيضًا كمشغل
checkbox-with-label = صندوق اختيار مع تسمية
checkbox-with-custom-icon-label = صندوق اختيار مع أيقونة مخصصة وتسمية
switch-basic = مفاتيح تبديل أساسية
switch-1 = مفتاح تبديل 1
switch-2 = مفتاح تبديل 2
select-placeholder = اختر خيارًا...

## Button group, card, and collapsible examples
button-accept = قبول
button-maybe = ربما
button-decline = رفض
button-top = أعلى
button-middle = وسط
button-bottom = أسفل
card-starter-title = خطة البداية
card-starter-description = للنماذج الأولية والتجارب السريعة
card-starter-price = 9$ / شهر
card-starter-feature-1 = حتى 3 مشاريع
card-starter-feature-2 = دعم المجتمع
card-starter-feature-3 = مساحات عمل مشتركة
card-choose-plan = اختر الخطة
card-team-title = خطة الفريق
card-team-description = تحكم أكبر لتطبيقات الإنتاج
card-team-price = 29$ / شهر
card-team-feature-1 = مشاريع غير محدودة
card-team-feature-2 = دعم ذو أولوية
card-team-feature-3 = تخصيص السمة
card-preview = معاينة
card-upgrade = ترقية
collapsible-toggle = تبديل الطي
collapsible-header = انقر للتوسيع أو طي هذا القسم
collapsible-content-long = يحتوي هذا اللوح على كتلة محتوى أطول. يوضح مكون القابل للطي مع عدة أسطر من النص التي يمكن إظهارها أو إخفاؤها بالنقر على العنوان أعلاه.
collapsible-content-short = يحتوي هذا اللوح على كتلة محتوى قصيرة يمكن إظهارها أو إخفاؤها.
action-cancel = إلغاء
action-save = حفظ

## Menu and menubar examples
menu-root = قائمة
menu-new = جديد
menu-shortcut-new = Ctrl + N
menu-open = فتح
menu-shortcut-open = Ctrl + O
menu-open-recent = فتح الأخير
menu-doc-1 = مستند 1
menu-doc-2 = مستند 2
menu-version-1 = الإصدار 1
menu-version-2 = الإصدار 2
menu-version-3 = الإصدار 3
menu-doc-3 = مستند 3
menu-save = حفظ
menu-save-as = حفظ باسم
menu-quit = إنهاء
menubar-file = ملف
menubar-edit = تحرير
menubar-view = عرض
menubar-help = مساعدة
menubar-cut = قص
menubar-copy = نسخ
menubar-paste = لصق
menubar-zoom-in = تكبير
menubar-zoom-out = تصغير
menubar-zoom-level = مستوى التكبير
menubar-zoom-10 = 10%
menubar-zoom-20 = 20%
menubar-zoom-50 = 50%
menubar-zoom-100 = 100%
menubar-zoom-150 = 150%
menubar-zoom-200 = 200%
menubar-show-license = عرض الترخيص
menubar-about = حول

## Tooltip and table examples
tooltip-placement-top-start = أعلى البداية
tooltip-placement-top = أعلى
tooltip-placement-top-end = أعلى النهاية
tooltip-placement-left-start = يسار البداية
tooltip-placement-left = يسار
tooltip-placement-left-end = يسار النهاية
tooltip-placement-right-start = يمين البداية
tooltip-placement-right = يمين
tooltip-placement-right-end = يمين النهاية
tooltip-placement-bottom-start = أسفل البداية
tooltip-placement-bottom = أسفل
tooltip-placement-bottom-end = أسفل النهاية
tooltip-placement-over = فوق
tooltip-placement-cursor = المؤشر
tooltip-text = هذه تلميحة
table-show-group = إظهار المجموعة
table-show-notes = إظهار الملاحظات
table-prioritize-status = إعطاء أولوية للحالة

## XY pad, radio button, and virtual table examples
xypad-heading = لوحة XY ثنائية الأبعاد
radiobutton-basic = أزرار اختيار أساسية
radiobutton-with-labels = أزرار اختيار مع تسميات
option-first = الأول
option-second = الثاني
option-third = الثالث
toggle-bold = عريض
virtual-table-heading = VirtualTable مجموعة بيانات كبيرة (5000 صف)
virtual-table-description = صفوف افتراضية بارتفاع ثابت لمجموعات البيانات الكبيرة. استخدم الفرز وتغيير الحجم لقياس التفاعل.

## Chip example
chip-label = رقاقة
chip-color-red = أحمر
chip-color-green = أخضر
chip-color-blue = أزرق

## Select example
select-option-one = واحد
select-option-two = اثنان
select-option-three = ثلاثة
select-option-four = أربعة
select-option-five = خمسة
select-option-six = ستة طويلة جدًا
select-option-seven = سبعة
select-option-eight = ثمانية
select-option-nine = تسعة
select-option-ten = عشرة
select-option-eleven = احدى عشر
select-option-twelve = اثنا عشر