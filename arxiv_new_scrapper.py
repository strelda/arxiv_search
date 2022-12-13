import pandas as pd
import arxiv
 
topic = input("Enter the topic you need to search for : ")
 
search = arxiv.Search(
  query = topic,
  max_results = 300,
  sort_by = arxiv.SortCriterion.SubmittedDate,
  sort_order = arxiv.SortOrder.Descending
)
 
all_data = []
for result in search.results():
  temp = ["","","","",""]
  temp[0] = result.title
  temp[1] = result.published
  temp[2] = result.entry_id
  temp[3] = result.summary
  temp[4] = result.pdf_url
  all_data.append(temp)
 
column_names = ['Title','Date','Id','Summary','URL']
df = pd.DataFrame(all_data, columns=column_names)
 
print("Number of papers extracted : ",df.shape[0])
df.head()