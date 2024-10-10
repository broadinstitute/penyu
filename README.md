# penyu

A Rust library to write some RDF as Turtle.

Yup, that is it. For now, at least.

I know what you are thinking. Why yet another RDF library? Surely writing some RDF as Turtle is pretty simple and 
straight-forward, so we are just reinventing the wheel here, right?

One would assume it is trivial. But I looked at a bunch of RDF libraries and the way they model RDF makes absolutely no 
sense to me.

I mean, I know we all want to be as generic as possible, but as far as I am concerned, subjects are entities. Yes, I 
know, it has been proposed to allow literals to be used as subjects. Like, twenty years ago. This proposal has never 
been adopted all this time, so I am going to gamble here that it never will be, and I think it is a silly idea anyway.

Regarding the name, 'penyu' (pronounced as "pen you", as in "The pen you use.") is the Indonesian/Malay word for 
'turtle'. Why Indonesian? Because it is a really cute language. Check it out. Also, 'penyu' is a nicely short and easy 
word, and in most other languages I know, the word for 'turtle' is longer and harder to pronounce for English speakers.