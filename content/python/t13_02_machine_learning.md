@@lesson t13-ml 🎓 Beyond IT: Machine Learning Fundamentals
@@concepts numbers, lists, functions
@@text
> **Beyond IT scope.** Machine learning engineering is its own career. This lesson explains the core ideas so you understand what ML tools do, and builds two classic algorithms from scratch so nothing is magic.

**Machine learning** means fitting a model to examples instead of writing the rules by hand. Two main kinds:

- **Supervised**: examples come with answers (labels). *Regression* predicts a number (disk usage next month); *classification* predicts a category (spam or not, failing drive or healthy).
- **Unsupervised**: no labels; find structure. *Clustering* groups similar items (similar servers by load pattern); *anomaly detection* flags the unusual.

## The workflow

```text
data ─▶ features (numbers) ─▶ split: train / test ─▶ fit on train ─▶ evaluate on test ─▶ deploy, monitor
```

- **Features**: the inputs as numbers (CPU %, age in days, error count). Categories become one-hot columns; scales are normalised.
- **Train/test split**: evaluate on data the model has **never seen**. A model that scores perfectly on its training data but badly on new data is **overfitting**: it memorised instead of learning.
- **Metrics**: for regression, error (MAE, RMSE) and R²; for classification, **accuracy**, **precision** (of the items flagged, how many were right) and **recall** (of the real positives, how many were found). With rare events (1 failing drive in 1000), accuracy alone lies: predicting "healthy" always is 99.9% accurate and useless.

## Linear regression

Fit `y = a·x + b` by **least squares**, minimising the squared errors:

```text
a = Σ(x − x̄)(y − ȳ) / Σ(x − x̄)²        b = ȳ − a·x̄
R² = 1 − Σ(y − ŷ)² / Σ(y − ȳ)²          (1 = perfect, 0 = no better than the mean)
```

## k-nearest neighbours

To classify a new point, find the **k** closest training points (Euclidean distance) and take a majority vote. No training step at all, which makes it easy to understand and a good baseline.

## The real tools

```python,norun
from sklearn.model_selection import train_test_split
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import classification_report

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.25, random_state=42)
model = RandomForestClassifier(n_estimators=200).fit(X_train, y_train)
print(classification_report(y_test, model.predict(X_test)))
```

- **scikit-learn**: classic ML (regression, trees, forests, clustering, pipelines).
- **PyTorch** (and JAX, TensorFlow): neural networks and deep learning.
- **Hugging Face** `transformers`: pre-trained language and vision models.
- For calling hosted large language models, you use their HTTP APIs and SDKs; the AI helper in this app is an example.

@@quiz
A model is 99% accurate on its training data and 60% on the test data. What's happening?
- Underfitting
* Overfitting: it memorised the training data instead of learning general patterns
- The test data is wrong
- It's working well
? Simplify the model, add data or regularise.

@@quiz
1 in 1000 drives fails. A model always predicts "healthy". What's its accuracy and recall?
* 99.9% accuracy but 0% recall: it never finds a failing drive
- 0% accuracy
- 100% recall
- 50% for both
? Use precision and recall for rare events.

@@quiz
Which task is classification?
- Predicting next month's disk usage in GB
* Predicting whether a login is suspicious or normal
- Grouping servers without labels
- Computing an average
? Classification predicts a category.

@@flashcards
Supervised learning :: learning from labelled examples
Regression :: predicts a number
Classification :: predicts a category
Clustering :: groups unlabelled data by similarity
Feature :: a numeric input to a model
Overfitting :: great on training data, poor on new data
Precision :: of the predicted positives, the share that were right
Recall :: of the actual positives, the share that were found
R² :: how much variance a regression explains (1 = perfect)
k-NN :: classify by majority vote of the k nearest examples

@@exercise Linear regression from scratch
Write `fit_line(xs, ys)` returning `(a, b)` for `y = a·x + b` by least squares, `predict(model, x)`, and `r_squared(model, xs, ys)`. Round nothing inside; the starter rounds when printing. If all `xs` are equal, raise `ValueError("need at least two different x values")`.

The data is disk usage (GB) over 6 months; the print forecasts month 12.
@@starter
def fit_line(xs, ys):
    pass


def predict(model, x):
    pass


def r_squared(model, xs, ys):
    pass


months = [1, 2, 3, 4, 5, 6]
used = [210, 228, 251, 266, 290, 305]
model = fit_line(months, used)
print([round(v, 2) for v in model], round(predict(model, 12), 1), round(r_squared(model, months, used), 4))
@@expected
[19.31, 190.73] 422.5 0.9968
@@tests
check(fit_line([0, 1, 2], [1, 3, 5]), (2.0, 1.0), "exact line")
check(r_squared((2.0, 1.0), [0, 1, 2], [1, 3, 5]), 1.0, "perfect fit")
try:
    fit_line([3, 3], [1, 2])
    check("fitted", "ValueError", "vertical")
except ValueError as e:
    check(str(e), "need at least two different x values", "message")
@@hint
`mx = sum(xs) / len(xs)` (same for y); `sxx = sum((x - mx) ** 2 for x in xs)`; `sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))`; `a = sxy / sxx`. For R², `ss_res = sum((y - predict(model, x)) ** 2 ...)` and `ss_tot = sum((y - my) ** 2 ...)`.
@@solution
def fit_line(xs, ys):
    mx = sum(xs) / len(xs)
    my = sum(ys) / len(ys)
    sxx = sum((x - mx) ** 2 for x in xs)
    if sxx == 0:
        raise ValueError("need at least two different x values")
    sxy = sum((x - mx) * (y - my) for x, y in zip(xs, ys))
    a = sxy / sxx
    return a, my - a * mx


def predict(model, x):
    a, b = model
    return a * x + b


def r_squared(model, xs, ys):
    my = sum(ys) / len(ys)
    ss_res = sum((y - predict(model, x)) ** 2 for x, y in zip(xs, ys))
    ss_tot = sum((y - my) ** 2 for y in ys)
    return 1 - ss_res / ss_tot


months = [1, 2, 3, 4, 5, 6]
used = [210, 228, 251, 266, 290, 305]
model = fit_line(months, used)
print([round(v, 2) for v in model], round(predict(model, 12), 1), round(r_squared(model, months, used), 4))

@@exercise k-NN and classification metrics
Write:

- `knn_predict(train, point, k=3)`: `train` is a list of `(features, label)` where features is a tuple of numbers. Return the majority label among the `k` nearest examples (Euclidean distance). Break vote ties by picking the label of the nearest example among the tied labels.
- `metrics(actual, predicted, positive)`: return `{"accuracy": ..., "precision": ..., "recall": ...}`, each rounded to 3 decimals; precision or recall is `0.0` when its denominator is zero.

Features here are (failed logins per hour, distinct countries); labels `"attack"` / `"normal"`.
@@starter
import math
from collections import Counter


def knn_predict(train, point, k=3):
    pass


def metrics(actual, predicted, positive):
    pass


train = [((1, 1), "normal"), ((2, 1), "normal"), ((0, 1), "normal"), ((3, 2), "normal"),
         ((40, 5), "attack"), ((25, 3), "attack"), ((60, 8), "attack")]
tests = [((2, 2), "normal"), ((30, 4), "attack"), ((12, 2), "attack"), ((5, 1), "normal")]
predicted = [knn_predict(train, p) for p, _ in tests]
print(predicted)
print(metrics([label for _, label in tests], predicted, "attack"))
@@expected
['normal', 'attack', 'normal', 'normal']
{'accuracy': 0.75, 'precision': 1.0, 'recall': 0.5}
@@tests
check(knn_predict([((0,), "a"), ((10,), "b")], (1,), k=1), "a", "k=1")
check(knn_predict([((0,), "a"), ((3,), "b"), ((10,), "c"), ((11,), "c")], (2,), k=2), "b", "tie goes to the nearest")
check(metrics(["n", "n"], ["n", "n"], "y"), {"accuracy": 1.0, "precision": 0.0, "recall": 0.0}, "no positives")
@@hint
Sort with `sorted(train, key=lambda ex: math.dist(ex[0], point))[:k]`. Count with `Counter`; find the best count, then walk the nearest list in order and return the first label whose count equals the best. For metrics count true positives (`a == p == positive`), predicted positives and actual positives.
@@solution
import math
from collections import Counter


def knn_predict(train, point, k=3):
    nearest = sorted(train, key=lambda ex: math.dist(ex[0], point))[:k]
    votes = Counter(label for _, label in nearest)
    best = max(votes.values())
    for _, label in nearest:
        if votes[label] == best:
            return label


def metrics(actual, predicted, positive):
    pairs = list(zip(actual, predicted))
    tp = sum(1 for a, p in pairs if a == p == positive)
    predicted_pos = sum(1 for _, p in pairs if p == positive)
    actual_pos = sum(1 for a, _ in pairs if a == positive)
    return {
        "accuracy": round(sum(1 for a, p in pairs if a == p) / len(pairs), 3),
        "precision": round(tp / predicted_pos, 3) if predicted_pos else 0.0,
        "recall": round(tp / actual_pos, 3) if actual_pos else 0.0,
    }


train = [((1, 1), "normal"), ((2, 1), "normal"), ((0, 1), "normal"), ((3, 2), "normal"),
         ((40, 5), "attack"), ((25, 3), "attack"), ((60, 8), "attack")]
tests = [((2, 2), "normal"), ((30, 4), "attack"), ((12, 2), "attack"), ((5, 1), "normal")]
predicted = [knn_predict(train, p) for p, _ in tests]
print(predicted)
print(metrics([label for _, label in tests], predicted, "attack"))
