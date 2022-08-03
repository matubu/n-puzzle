/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.c                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mberger- <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2022/08/03 02:24:48 by mberger-          #+#    #+#             */
/*   Updated: 2022/08/03 02:24:49 by mberger-         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <unistd.h>

int	r(char *b, int i, int c, int first)
{
	while (++c <= 9 && i != b[0])
		(void)((b[i + 3] = c + '0') + r(b, i + 1, c, first-- == 1));
	return (i == b[0] && write(1, b + 1 + 2 * first, b[0] + 2 * !first));
}

void	ft_print_combn(int n)
{
	(void)(r((char *)((int [9]){n | 2108416}), 0, -1, 1) + write(1, "\n", 1));
}

int	main(void)
{
	int	i;

	i = 0;
	while (++i < 10)
		ft_print_combn(i);
}
