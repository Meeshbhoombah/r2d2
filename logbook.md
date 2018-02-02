
# Research
1. How to use user data to train model?
    - Intuited approach: save conversations and use the parsed context to append user's 
      responses to training data for chatbot
2. How can we give the bot a personality?
    - Can we just dump in a corpus of words from a selected personality
3. Is it a bad idea to use RNN to try to generate responses to questions unknown?
4. How to parse out an unknown user response?
    - I.e: we give a broad generalized question ot the user about

## [RNN](https://towardsdatascience.com/personality-for-your-chatbot-with-recurrent-neural-networks-2038f7f34636)
- RNN = Recurrent Neural Network
    + Deep learning model, handles sequences
    + 

## [Deep Learning for Chatbots](http://www.wildml.com/2016/07/deep-learning-for-chatbots-2-retrieval-based-model-tensorflow/et)
Two model's for chatbots:
- Retrival-based
    + Repository of pre-defined responses
    + Input:
        * Context (c) - conversation up until this point
        * Potential response (r)
            - Model calculates score for the response
            - Good response = calculate multiple scores, choose highest
- Generative-based
    + Generates responses never seen before
    + Does not work well in practice
        * Grammatical mistakes
        * Irrelevant, generic, inconsistent respones

Most system's are either retrival based or a hybrid of retrival/generative. Ex: Google's Smart
Reply.

Evaluating models:
- Commonly uses `recall@k` - let model pick *k* best responses out of 10 possible, 1 true,
  9 distractors
    + If correct one is chosen, mark test as correct
    + Larger k = tasks become easier
    + k = 10, 100% recall
- Distractors can be selected randomly from a dataset, but in the real world there are 
  1,000,000s of possibilities
    + Cannot evaluate this many
    + [Google Smart Reply uses clustering techniques](https://arxiv.org/abs/1606.04870)

## [Do it Yourself NLP](https://medium.com/rasa-blog/do-it-yourself-nlp-for-bot-developers-2e2da2817f3d)
- Use word vectors to capture the meaning of a word and parse user message
    + word embeddings like word2vec or GloVe can represent text data
    + Example:
        * Provide some cuisine's as context, look for most similar words in sentence
        * Look through words and select one whose average cosine similar to references words
          is above threshhold
- Use words vectors to capture generalized user intent
    + Many ways to combine word vectors to represent a sentence -- ???
    + Add them up:
        * May be abhorrent -- ???
    + Create "bag of words" vectors for each, then categorize using a "distance criterian" -- ???
- Use SpaCy / textacy to clean up/parse text
- scikit-learn to build models
- MITIE can do everything, has Python bindings
    + Sophistaced, uses spectral word embeddings -- ???
    + Categorizer = straightforward SVM, entity recognizer uses structural SVM -- ???

Text Categorizer
```python
import sys, os
from mitie import *

trainer = text_categorizer_trainer("/path/to/total_word_feature_extractor.dat")

data = {} # same as before  - omitted for brevity

for label in training_examples.keys():
  for text in training_examples[label]["examples"]:
    tokens = tokenize(text)
    trainer.add_labeled_text(tokens,label)
    
trainer.num_threads = 4
cat = trainer.train()

cat.save_to_disk("my_text_categorizer.dat")

# we can then use the categorizer to predict on new text
tokens = tokenize("somewhere that serves chinese food")
predicted_label, _ = cat(tokens)
```

Entitiy Recognizer
```python
import sys, os
from mitie import *
sample = ner_training_instance(["I", "am", "looking", "for", "some", "cheap", "Mexican", "food", "."])

sample.add_entity(xrange(5,6), "pricerange")
sample.add_entity(xrange(6,7), "cuisine")

# And we add another training example
sample2 = ner_training_instance(["show", "me", "indian", "restaurants", "in", "the", "centre", "."])
sample2.add_entity(xrange(2,3), "cuisine")
sample2.add_entity(xrange(6,7), "area")


trainer = ner_trainer("/path/to/total_word_feature_extractor.dat")

trainer.add(sample)
trainer.add(sample2)

trainer.num_threads = 4

ner = trainer.train()

ner.save_to_disk("new_ner_model.dat")


# Now let's make up a test sentence and ask the ner object to find the entities.
tokens = ["I", "want", "expensive", "korean", "food"]
entities = ner.extract_entities(tokens)


print "\nEntities found:", entities
print "\nNumber of entities detected:", len(entities)
for e in entities:
    range = e[0]
    tag = e[1]
    entity_text = " ".join(tokens[i] for i in range)
    print "    " + tag + ": " + entity_text
    
# output 
# >>> Number of entities detected: 2
# >>>     pricerange: expensive
# >>>     cuisine: korean
```   
 
