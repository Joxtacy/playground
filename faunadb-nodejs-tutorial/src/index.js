const express = require("express")();
const faunadb = require("faunadb");

const PORT = 5050;

const {
    Call,
    Collection,
    Create,
    Function: Fn,
    Get,
    Index,
    Join,
    Lambda,
    Match,
    Paginate,
    Ref,
    Select,
    Var,
} = faunadb.query;

const db = new faunadb.Client({
    secret: "fnAD4cFhlTACCLdKBOfdj38HjIQ8mRnOFyLTB0d0",
});

express.get("/tweet", async (res, req) => {
    const docs = db.query(
        Paginate(Match(Index("tweets_by_user"), Call(Fn("getUser"), "Joxtacy")))
    );
    res.send(docs);
});
express.get("/tweet/:id", async (req, res) => {
    const doc = await db.query(Get(Ref(Collection("tweets"), req.params.id)));
    res.send(doc);
});
express.get("/feed", async (req, res) => {
    const docs = await db.query(
        Paginate(
            Join(
                Match(
                    Index("followees_by_follower"),
                    Call(Fn("getUser"), "Bob")
                ),
                Index("tweets_by_user")
            )
        )
    );

    res.send(docs);
});

express.post("/tweet", async (req, res) => {
    const data = {
        user: Call(Fn("getUser"), "Joxtacy"),
        text: "こんにちは！",
    };
    const doc = await db.query(Create(Collection("tweets"), { data }));

    res.send(doc);
});

express.post("/relationship", async (req, res) => {
    const data = {
        follower: Call(Fn("getUser"), "Bob"),
        followee: Call(Fn("getUser"), "Joxtacy"),
    };

    const doc = await db.query(Create(Collection("relationships"), { data }));

    res.send(doc);
});

express.listen(PORT, () =>
    console.log(`App is now listening on 'http://localhost:${PORT}/'`)
);
