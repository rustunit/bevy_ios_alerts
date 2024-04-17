//
//  Header.h
//  
//
//  Created by Stephan on 15.04.24.
//

#ifndef Header_h
#define Header_h

@import UIKit;

@interface PopupsManager : NSObject
+ (void) unregisterAllertView;
@end

void popup_message_click(void);
void popup_dialog_click(uint8_t);
void popup_input_click(char*);

#endif /* Header_h */
