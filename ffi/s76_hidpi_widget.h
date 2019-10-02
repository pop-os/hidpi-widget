#include <gtk/gtk.h>

typedef struct { } HiDpiToggle;

HiDpiToggle *hidpi_toggle_new (void);

GtkWidget *hidpi_toggle_widget (const HiDpiToggle *self);

void hidpi_toggle_free (HiDpiToggle *self);
