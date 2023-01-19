import { trpc } from '@lib/trpc'
import type { NextPage } from 'next'

import { Box, Button, Flex, Heading, HStack, VStack, Text } from '@chakra-ui/react'
import { DefaultLayout } from '@components/layouts'
import { ThemeToggler } from '@components/shared'

const HomePage: NextPage = () => {

	const getRandom = trpc.random.getRandom.useQuery()

	return (<>

		<DefaultLayout
			title='Home'
		>

			<ThemeToggler position='absolute' top='2em' right='2em' size='lg'/>

			<VStack
				direction='column'
				align='center'
				justify='center'
				w='100%'
				h='100vh'
				spacing={10}
			>

				<Text fontSize='1.5em'>
					Projet Docker - Admin virtualisation
				</Text>

				<Text>
					Par Bartholomé GILI (Devops 1)
				</Text>


				<Box fontSize='xl' fontWeight='black'>
					{getRandom.data}
				</Box>

				<Button
					onClick={() => getRandom.refetch()}
				>
					Refresh
				</Button>
			</VStack>


		</DefaultLayout>		
	
	</>)
}

export default HomePage