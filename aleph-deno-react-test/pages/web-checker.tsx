import React, { useState } from "react";
import axios from "https://esm.sh/axios";
import { useDeno } from "https://deno.land/x/aleph/mod.ts";

export default function App() {
    const [siteURL, setSiteURL] = useState("");
    const [response, setResponse] = useState(undefined);
    const RAPIDAPI_KEY = useDeno(() => Deno.env.get("RAPIDAPI_KEY"));

    const submitData = (siteURL: string) => {
        setResponse("Loading...");
        const options = {
            params: { siteURL },
            headers: {
                "x-rapidapi-key": RAPIDAPI_KEY,
                "x-rapidapi-host": "website-data-gathering-and-update-tracking.p.rapidapi.com",
            }
        };

        axios
            .get(
                "https://website-data-gathering-and-update-tracking.p.rapidapi.com/sitecheck",
                options
            )
            .then(function (response) {
                setResponse(response.data);
                console.info(response.data);
            })
            .catch(function (error) {
                setResponse("Error fetching data");
                console.error(error);
            });
    };

    return (
        <div style={{ fontFamily: "sans-serif", textAlign: "center" }}>
            <h1>Is it Down?</h1>
            <h2>
                Go{" "}
                <a href="https://rapidapi.com/jakash1997/api/website-data-gathering-and-update-tracking" target="_blank" >here</a>
                {" "}to get an API key.
            </h2>
            <form onSubmit={(e) => {
                e.preventDefault();
                submitData(siteURL);
            }}>
                <input value={siteURL} onChange={(e) => setSiteURL(e.target.value)} type="text" />
                <button type="submit">Submit</button>
            </form>
            <br />
            <code>{JSON.stringify(response, null, 4)}</code>
        </div>
    );
}
