'''
Quora Questions Data ... Download train.csv from: https://www.kaggle.com/c/quora-question-pairs/data
'''
from sys import argv
import gensim
import pandas as pd

filename = argv[1]
output   = argv[2] 
df       = pd.read_csv(filename)

def read_questions(row,column_name):
    return gensim.utils.simple_preprocess(str(row[column_name]).encode('utf-8'))

with open(output, 'w') as fp:
    for index, row in df.iterrows():        
        question = " ".join(read_questions(row, "question1"))
        fp.write("{}\n".format(question))
        if row["is_duplicate"] == 0:
            question = " ".join(read_questions(row, "question2"))
            fp.write("{}\n".format(question))
                            
