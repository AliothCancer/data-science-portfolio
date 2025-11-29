



import pandas as pd
from ydata_profiling import ProfileReport

df = pd.read_csv("../house-prices-advanced-regression-techniques/train.csv")
profile = ProfileReport(df, title="YData Profiling Report")

profile.to_file("your_report.html")
