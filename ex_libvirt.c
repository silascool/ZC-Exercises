#include <libvirt/libvirt.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * Used the example from virConnectListAllDomains documentation as a baseline.
 * Then i used virDomainGetID, virDomainGetState, virDomainGetName to get the
 * needed information.
 */

void int_to_state(int state, const char **str_state){
    switch (state) {
        case 0:
            *str_state = "no state";
            break;
        case 1:
            *str_state = "running";
            break;
        case 2:
            *str_state = "blocked";
         break;
        case 3:
            *str_state = "paused";
            break;
        case 4:
            *str_state = "shut down";
            break;
        case 5:
            *str_state = "crashed";
            break;
        case 6:
            *str_state = "suspended";
            break;
        default:
            break;
    }
}

int main() {
    virDomainPtr *domains;
    int state;
    const char *str_state;
    virConnectPtr c = virConnectOpen(NULL);
    unsigned int flags = VIR_CONNECT_LIST_DOMAINS_RUNNING | VIR_CONNECT_LIST_DOMAINS_PERSISTENT;
    int ret = virConnectListAllDomains(c, &domains, flags);
    if (ret < 0)
        exit(-1);
    //printing names of coloumns 
    printf("%-8s %-15s %-15s\n", "ID", "Name", "State");
    printf("-----------------------------------\n");
    for (int i = 0; i < ret; i++) {
        //getting information from domains[i]
        int id = virDomainGetID(domains[i]);
        virDomainGetState(domains[i], &state, NULL, 0);
        int_to_state(state, &str_state);
        const char *name = virDomainGetName(domains[i]);
        printf("%-8d %-15s %-15s\n", id, name, str_state);
        virDomainFree(domains[i]);
    }
    free(domains);
    exit(0);
}