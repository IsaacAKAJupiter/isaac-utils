<script lang="ts">
    import { addAlert } from '../../../stores/alert';
    import { configStore } from '../../../stores/config';
    import {
        getConfigCopy,
        writeConfig,
        type ConfigP2PContact,
    } from '../../../util/config';
    import Icon from '../../icon.svelte';

    let editDialogElement = $state<HTMLDialogElement>();
    let newDialogElement = $state<HTMLDialogElement>();
    let originalEditingContact = $state<ConfigP2PContact>();
    let editingContact = $state<ConfigP2PContact>();
    let newContact = $state<ConfigP2PContact>({ ip: '', name: '' });
    let dialogError = $state<string>();

    async function removeContact(contact: ConfigP2PContact) {
        if ($configStore) {
            let newConfig = getConfigCopy($configStore);
            newConfig.p2p.contacts = newConfig.p2p.contacts.filter(
                (c) => c.ip !== contact.ip
            );
            configStore.set(newConfig);
            await writeConfig(newConfig);
        } else {
            addAlert({
                type: 'info',
                message: 'Could not remove the contact.',
                dismissible: true,
                timeout: 5000,
            });
        }
    }

    async function saveContact() {
        if (!editingContact || !originalEditingContact) return;

        if (!editingContact.ip) {
            dialogError = 'IP must not be empty.';
            return;
        }

        if (!editingContact.name) {
            dialogError = 'Name must not be empty.';
            return;
        }

        if ($configStore) {
            let newConfig = getConfigCopy($configStore);
            const contact = newConfig.p2p.contacts.find(
                (c) =>
                    c.ip === originalEditingContact!.ip &&
                    c.name === originalEditingContact!.name
            );
            if (contact) {
                contact.ip = editingContact.ip;
                contact.name = editingContact.name;
            }
            configStore.set(newConfig);
            await writeConfig(newConfig);
            editDialogElement?.close();
        } else {
            addAlert({
                type: 'info',
                message: 'Could not save the contact.',
                dismissible: true,
                timeout: 5000,
            });
        }
    }

    async function saveNewContact() {
        if (!$configStore) return;

        if (!newContact.ip) {
            dialogError = 'IP must not be empty.';
            return;
        }

        if (!newContact.name) {
            dialogError = 'Name must not be empty.';
            return;
        }

        let newConfig = getConfigCopy($configStore);

        if (newConfig.p2p.contacts.some((c) => c.ip === newContact.ip)) {
            dialogError = 'IP already used in another contact.';
            return;
        }

        newConfig.p2p.contacts = [
            ...newConfig.p2p.contacts,
            { ip: newContact.ip, name: newContact.name },
        ];
        configStore.set(newConfig);
        await writeConfig(newConfig);
        newDialogElement?.close();
    }

    function editContact(contact: ConfigP2PContact) {
        editingContact = { ...contact };
        originalEditingContact = { ...contact };
        dialogError = '';
        editDialogElement?.showModal();
    }

    function startNewContact() {
        newContact = {
            ip: '',
            name: '',
        };
        dialogError = '';
        newDialogElement?.showModal();
    }
</script>

<div class="p-4 bg-accent">
    <div>
        <p class="mb-4 text-sm">
            Contacts can be used for a couple quality of life features. You can
            easily send to files/text to contacts without having to remember
            their IP address. If file/text is received, you will also get the
            contact name if it matches one for the peer.
        </p>

        <div class="mb-4">
            <button class="main-btn" onclick={() => startNewContact()}>
                New Contact
            </button>
        </div>

        <table class="table-fixed w-full">
            <thead>
                <tr>
                    <th class="text-left">Name</th>
                    <th>IP</th>
                    <th>&nbsp;</th>
                </tr>
            </thead>
            <tbody>
                {#each $configStore?.p2p.contacts ?? [] as contact}
                    <tr>
                        <td class="text-left">{contact.name}</td>
                        <td class="text-center">
                            {contact.ip}
                        </td>
                        <td class="text-right">
                            <div
                                class="flex space-x-2 items-center justify-end"
                            >
                                <button
                                    class="w-fit text-2xl cursor-pointer text-primary"
                                    onclick={() => editContact(contact)}
                                >
                                    <Icon name="edit" />
                                </button>
                                <button
                                    class="w-fit text-2xl cursor-pointer text-red-600"
                                    onclick={() => removeContact(contact)}
                                >
                                    <Icon name="x" />
                                </button>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</div>

<dialog
    class="border-none p-0 bg-transparent max-h-[90vh] overflow-y-auto"
    bind:this={editDialogElement}
>
    {#if editingContact}
        <div class="relative bg-bg p-4 text-white rounded-lg">
            <div class="absolute top-2 right-2">
                <button
                    class="w-fit text-2xl cursor-pointer"
                    onclick={() => editDialogElement?.close()}
                >
                    <Icon name="x" />
                </button>
            </div>

            <p class="text-center text-xl mb-4">Edit Contact</p>

            <div class="flex space-x-2 items-center mb-4 max-w-96">
                <div class="flex items-center space-x-1 w-28">
                    <p>Contact Name:</p>
                </div>
                <input
                    class="input !w-auto flex-1"
                    type="text"
                    bind:value={editingContact.name}
                />
            </div>
            <div class="flex space-x-2 items-center mb-8 max-w-96">
                <p class="w-28">Custom IP:</p>
                <input
                    class="input !w-auto flex-1"
                    type="text"
                    bind:value={editingContact.ip}
                />
            </div>

            {#if dialogError}
                <p class="text-right text-red-600">{dialogError}</p>
            {/if}
            <div
                class="flex flex-col space-y-2 justify-center items-center sm:flex-row sm:justify-end sm:space-y-0 sm:space-x-4"
            >
                <button
                    class="red-btn"
                    onclick={() => editDialogElement?.close()}
                >
                    Cancel
                </button>
                <button class="main-btn" onclick={() => saveContact()}>
                    Save
                </button>
            </div>
        </div>
    {/if}
</dialog>

<dialog
    class="border-none p-0 bg-transparent max-h-[90vh] overflow-y-auto"
    bind:this={newDialogElement}
>
    <div class="relative bg-bg p-4 text-white rounded-lg">
        <div class="absolute top-2 right-2">
            <button
                class="w-fit text-2xl cursor-pointer"
                onclick={() => newDialogElement?.close()}
            >
                <Icon name="x" />
            </button>
        </div>

        <p class="text-center text-xl mb-4">New Contact</p>

        <div class="flex space-x-2 items-center mb-4 max-w-96">
            <div class="flex items-center space-x-1 w-28">
                <p>Contact Name:</p>
            </div>
            <input
                class="input !w-auto flex-1"
                type="text"
                bind:value={newContact.name}
            />
        </div>
        <div class="flex space-x-2 items-center mb-8 max-w-96">
            <p class="w-28">Custom IP:</p>
            <input
                class="input !w-auto flex-1"
                type="text"
                bind:value={newContact.ip}
            />
        </div>

        {#if dialogError}
            <p class="text-right text-red-600">{dialogError}</p>
        {/if}
        <div
            class="flex flex-col space-y-2 justify-center items-center sm:flex-row sm:justify-end sm:space-y-0 sm:space-x-4"
        >
            <button class="red-btn" onclick={() => newDialogElement?.close()}>
                Cancel
            </button>
            <button class="main-btn" onclick={() => saveNewContact()}>
                Save
            </button>
        </div>
    </div>
</dialog>

<style>
    ::backdrop {
        background-color: rgba(0, 0, 0, 0.6);
    }
</style>
