(def {fib}
	(fun {n}
	{cond
		{(or? (eq? n 1) (eq? n 2)) 1}
		{else (+ (fib (- n 1)) (fib (- n 2)))}
	}
))

(print (fib 2))