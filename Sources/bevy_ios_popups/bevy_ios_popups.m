#import "bevy_ios_popups.h"

@implementation PopupsManager
static UIAlertController* _currentAllert = nil;
+ (void) unregisterAllertView {
    if(_currentAllert != nil) {
        _currentAllert = nil;
    }
}
+(void) dismissCurrentAlert {
    if(_currentAllert != nil) {
        [_currentAllert dismissViewControllerAnimated:NO completion:nil];
        _currentAllert = nil;
    }
}
+ (void) showDialog: (NSString *) title message: (NSString*) msg yesTitle:(NSString*) b1 noTitle: (NSString*) b2{
    [PopupsManager dismissCurrentAlert];

    UIAlertController *alertController = [UIAlertController alertControllerWithTitle:title message:msg preferredStyle:UIAlertControllerStyleAlert];
    UIAlertAction *yesAction = [UIAlertAction actionWithTitle:b1 style:UIAlertActionStyleDefault handler:^(UIAlertAction * _Nonnull action) {
    [PopupsManager unregisterAllertView];
        popup_dialog_click(0);
    }];
    UIAlertAction *noAction = [UIAlertAction actionWithTitle:b2 style:UIAlertActionStyleDefault handler:^(UIAlertAction * _Nonnull action) {
    [PopupsManager unregisterAllertView];
        popup_dialog_click(1);
    }];
    [alertController addAction:yesAction];
    [alertController addAction:noAction];
   
    [[[[UIApplication sharedApplication] keyWindow] rootViewController] presentViewController:alertController animated:YES completion:nil];
    
    _currentAllert = alertController;
}

+(void)showMessage: (NSString *) title message: (NSString*) msg okTitle:(NSString*) b1 {
    [PopupsManager dismissCurrentAlert];
    
    UIAlertController *alertController = [UIAlertController alertControllerWithTitle:title message:msg preferredStyle:UIAlertControllerStyleAlert];
    UIAlertAction *okAction = [UIAlertAction actionWithTitle:b1 style:UIAlertActionStyleDefault handler:^(UIAlertAction * _Nonnull action) {
    [PopupsManager unregisterAllertView];
        popup_message_click();
    }];
    [alertController addAction:okAction];
    
    [[[[UIApplication sharedApplication] keyWindow] rootViewController] presentViewController:alertController animated:YES completion:nil];
    
    _currentAllert = alertController;
}

+(void)showInput: (NSString *) title message: (NSString*) msg okTitle:(NSString*) b1 placeholder:(NSString*) placeholder {
    [PopupsManager dismissCurrentAlert];
    
    UIAlertController *alertController = [UIAlertController alertControllerWithTitle:title message:msg preferredStyle:UIAlertControllerStyleAlert];
    UIAlertAction *okAction = [UIAlertAction actionWithTitle:b1 style:UIAlertActionStyleDefault handler:^(UIAlertAction * _Nonnull action) {
        [PopupsManager unregisterAllertView];
        NSString* text = [[alertController textFields][0] text];
        const char *c_text = [text UTF8String];
        popup_input_click(c_text);
    }];
    [alertController addAction:okAction];
    [alertController addTextFieldWithConfigurationHandler:^(UITextField *textField) {
        textField.placeholder = placeholder;
    }];
    
    [[[[UIApplication sharedApplication] keyWindow] rootViewController] presentViewController:alertController animated:YES completion:nil];
    
    _currentAllert = alertController;
}
@end

void ios_popup_dialog(char* title, char* message, char* yes, char* no) {
    NSString *ns_title = [NSString stringWithUTF8String:title];
    NSString *ns_msg = [NSString stringWithUTF8String:message];
    NSString *ns_yes = [NSString stringWithUTF8String:yes];
    NSString *ns_no = [NSString stringWithUTF8String:no];
    
     [PopupsManager showDialog:ns_title message:ns_msg yesTitle:ns_yes noTitle:ns_no];
}
void ios_popup_message(char* title, char* message, char* ok) {
    NSString *ns_title = [NSString stringWithUTF8String:title];
    NSString *ns_msg = [NSString stringWithUTF8String:message];
    NSString *ns_ok = [NSString stringWithUTF8String:ok];
    
    [PopupsManager showMessage:ns_title message:ns_msg okTitle:ns_ok];
}
void ios_popup_input(char* title, char* message, char* ok, char* placeholder) {
    NSString *ns_title = [NSString stringWithUTF8String:title];
    NSString *ns_msg = [NSString stringWithUTF8String:message];
    NSString *ns_ok = [NSString stringWithUTF8String:ok];
    NSString *ns_placeholder = [NSString stringWithUTF8String:placeholder];
    
    [PopupsManager showInput:ns_title message:ns_msg okTitle:ns_ok placeholder:ns_placeholder];
}
void ios_popup_dismiss_current(void) {
    [PopupsManager dismissCurrentAlert];
}
