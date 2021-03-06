#include <czmq.h>

int main(int argc, char const *argv[])
{
	zsock_t *rep = zsock_new_rep("tcp://127.0.0.1:9090");

	char *string = zstr_recv(rep);
	FILE *fp =  fopen(argv[1], "w");
	//Saida para o console.
	puts(string);
	//Saida pra o arquivo texto.
	fputs(string,fp);

	fclose(fp);
    zstr_free(&string);
	zsock_destroy(&rep);
	return 0;
}