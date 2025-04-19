use crate::module_bindings::chat_message_table::{ChatMessage, ChatMessageTableAccess};
use crate::module_bindings::player_table::PlayerTableAccess;
use crate::module_bindings::SubscriptionEventContext;
use crate::module_bindings::{DbConnection, ErrorContext};
use spacetimedb_sdk::{credentials, DbContext, Error, Identity, Table};

const HOST: &str = "http://localhost:3000";
const DB_NAME: &str = "guildmaster";

fn start() {
    // Connect to the database
    let ctx = connect_to_db();

    // Register callbacks to run in response to database events.
    register_callbacks(&ctx);

    // Subscribe to SQL queries in order to construct a local partial replica of the database.
    subscribe_to_tables(&ctx);

    // Spawn a thread, where the connection will process messages and invoke callbacks.
    ctx.run_threaded();

    // Handle CLI input
    user_input_loop(&ctx);
}

pub(crate) fn connect_to_db() -> DbConnection {
    DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(creds_store().load().expect("Error loading credentials"))
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect")
}

fn creds_store() -> credentials::File {
    credentials::File::new("quickstart-chat")
}

/// Our `on_connect` callback: save our credentials to a file.
fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

/// Our `on_connect_error` callback: print the error, then exit the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}

/// Register all the callbacks our app will use to respond to database events.
fn register_callbacks(ctx: &DbConnection) {
    // Register callback for player table insert events
    ctx.db().player().on_insert(|event_ctx, row| {
        // Example: Print new player info
        println!("Player joined: {:?}", row);
    });
    // Register callback for chat message table insert events
    ctx.db().chat_message().on_insert(|event_ctx, row| {
        // Example: Print new chat message
        println!("New chat message: {:?}", row);
    });
}

fn print_message(_ctx: &SubscriptionEventContext, message: &ChatMessage) {
    // Customize as needed; basic example:
    println!("[{}] {}: {}", message.timestamp, message.sender_id, message.text);
}

/// Our `on_subscription_applied` callback:
/// sort all past messages and print them in timestamp order.
fn on_sub_applied(ctx: &SubscriptionEventContext) {
    let mut messages = ctx.db.chat_message().iter().collect::<Vec<_>>();
    messages.sort_by_key(|m| m.timestamp);
    for message in messages {
        print_message(ctx, &message);
    }
    println!("Fully connected and all subscriptions applied.");
    println!("Use /name to set your name, or type a message!");
}

/// Or `on_error` callback:
/// print the error, then exit the process.
fn on_sub_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Subscription failed: {}", err);
    std::process::exit(1);
}

// Subscribes to tables to construct a local partial replica of the database.
fn subscribe_to_tables(ctx: &DbConnection) {
    ctx.subscription_builder()
        .on_applied(on_sub_applied)
        .on_error(on_sub_error)
        .subscribe([
            "SELECT * FROM player",
            "SELECT * FROM chat_message"
        ]);
}

// Handles CLI user input loop.
fn user_input_loop(_ctx: &DbConnection) {
    // Placeholder for CLI input loop
    // Implement as needed for your application
}