from transformers import AutoTokenizer, AutoModelForSeq2SeqLM

# Load the tokenizer and model
tokenizer = AutoTokenizer.from_pretrained("Helsinki-NLP/opus-mt-zh-en")
model     = AutoModelForSeq2SeqLM.from_pretrained("Helsinki-NLP/opus-mt-zh-en")

# Translate a simple sentence
src_text = "在某一年的十二月，我心情不好、想要去別的城市走走，所以我就坐上火車，從台北來到台南"
inputs   = tokenizer(src_text, return_tensors="pt")
outputs  = model.generate(**inputs, max_length=100)
print(tokenizer.decode(outputs[0], skip_special_tokens=True))
