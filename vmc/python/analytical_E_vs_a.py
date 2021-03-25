alpha = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
particles = [1, 10, 100, 500]
dim = [1, 2, 3]

local_energy = []

#new_pos = map(lambda _: spread * random_sample - 0.5, list(range(n_dims)))

""" for dim in dim:
    for particles in particles:
        for alpha in alpha:
            local_energy.append  """


#analytical wavefunction in one dimension


ExactEnergies = 0.25*(AlphaValues*AlphaValues+1.0/(AlphaValues*AlphaValues))
ExactVariance = 0.25*(1.0+((1.0-AlphaValues**4)**2)*3.0/(4*(AlphaValues**4)))-ExactEnergies*ExactEnergies
