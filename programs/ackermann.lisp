(def {a} (fun {m n} {
	cond
		{(zero? m) (+ n 1)}
		{else
			(cond
				{(zero? n) (a (- m 1) 1)}
				{(eq? m 1) (+ n 2)}
				{(eq? m 2) (+ (* 2 n) 3)}
				{else (a (- m 1) (a m (- n 1)))}
			)
		}
}))