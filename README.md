# Chainmail

A universal format to read, edit, and write to Minecraft world saves!

Going to work on building an intermediate format using the findings from my three (four) demo world parsing projects for LCE, Bedrock, and Java saves. Those include MCRegionJS (Gamedata-Parser too), Bedrock-LevelDB, and MCA-Buffer.

## Origins

Originally this was going to be done in TypeScript, with TypeScript type definitions. I am now conceptualizing how using Rust could very much make this an easier process to set up. Not even solely for the native part of things, speed, or that sort of thing. If anything, it's the strictness and expressiveness of things like enums and being able to use types inside of them. How this will work will become more apparent once I have everything here, in context. First I plan to port my Region-Types definitions over to Rust structs. Then I will be able to implement `TryFrom` for each of them, allowing to fully convert between any and all versions on the fly, be it for entity NBT, blocks, what have you. The roadblock is still to get all of the necessary information from each version in order to declare these properly. They need to map all and any of what the game would be put into the APIs here, so it essentially needs to perfectly match the "game's standard library", if you will. This is the most challenging part. The other part is that not all of the would formats are parseable yet, at least for more "obscure" world save types (ie. Legacy Console Edition, and all of it's world save format versions).

## Backstory

(this was done with dictation)

OK, this is my first time doing this. Well, intentionally.

Minecraft world conversion has been in a strong place of my heart for a very long time. It stems from challenges, like not being able to continue playing my worlds after certain editions were canceled, or if a new edition seemed more interesting to me at the time.

My goal with this project covers a multitude of different facets for me. The face value one, is to be able to convert Minecraft worlds between certain editions, understandably. The underlying ones though, however, maybe even more important to me than the main one. It's what this project represents.

I love programming. I also very much value the other people in programming, and I would very much like to be able to make programming a larger part of my life, than it already is.

Tackling a project like this seems ridiculous, and in hindsight, it probably is more effort than it's worth, especially since there are already tools out there, which do this for us. But again, the face value goal for making this project isn't the only reason why I am doing it. It's because it's what allows me to learn how to program, and that's something priceless. Let alone, being able to have an open source converter, but it also teaches me the fundamentals of what I need to make this my career, with time.

Over the last few days, I have started looking into things that might be holding me back into making this my career. And it seems to be personal struggles, rather than my programming abilities (not to sound cool or anything, lol). I think what has been holding me back in that regard is my lack of self-confidence to be able to share my work, and think that it will be of help to others too.

I also have learned that I have not been fully aware of how I structure my time, and plant things. A lot of other people I know seem to structure things with calendars, lists, and things like that. I seem to be driven more by what seems right in the moment, and what will work for me, and what I can accomplish in the meantime. I now see, that once again, this is not set in stone for everybody. I had this realization with music theory a few weeks ago as well, that it isn't the only way to do things. It's just a tool that one can utilize to get things done. This is a beautiful thing.

So maybe it isn't my programming skills, or my personality that's keeping me from being able to do this. It might just be my lack of self knowledge around how I plan and structure my time. I now see that this is something I absolutely can expand upon, and now I am very proud and excited too. It used to be something that I saw was lacking in myself, but now I see it as somewhere that I can grow into.

I also think I have had a lack of vision for myself in the field, feel like I wouldn't fit in. I have been more and more interested in tech minimalism lately, and not using social media much. I worry that this contrast would not work well in a place that is fundamentally centered around that. But now I see that this might just be an irrational fear. I don't know for certain what teams will be like, and likely this varies by team anyways. I absolutely feel I have something to contribute here, but I didn't believe in myself to be able to do that. Now I am learning that making myself known isn't something that I need to do to get a job, but it's the next step in giving yourself the credit for what you do. You can't always just get that from others. You have to give credit to yourself as well.

I am now seeing that this possible different outlook on things might just be the very thing which is unique to me, and "unique" is what you want, as a person in general, not even for simply getting into your desired career. Everyone has their unique qualities, so it's nothing to be afraid or ashamed of. Learning to embrace this has been one of the hardest things, yet one of the most fulfilling I've ever had.

What's the road map from here? The plan is to become proud of the work I have done, and show people that. It's not for my own personal gain, it's because it could be of help to others, and it would be a disservice to me and them, if I didn't make that work known. It will help everybody in that situation.

I am going to continue looking into conferences, and publicly sharing my opinions and thoughts, with the goal of making it more plainly visible. Getting feedback about your opinions from others is also a very beneficial, and it also goes both ways. Sometimes you might have misconceptions, sometimes they might have misconceptions, this is the great thing about dialogue.

Feel free to continue this dialogue with me, send me a message, comment on my YouTube videos, whatever you like! What's next on your plans for your life? I would love to hear about it! I bet you have something to teach me about a thing or two, as well :-)

Brandon
